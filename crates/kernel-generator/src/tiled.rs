//! Cache-tiled scalar convolution. Packed weights and folded bias are prepared
//! offline. Padding is materialized with the original INT8 zero point, making
//! bias folding exact even at borders. No heap, SIMD or assembly is required.
use crate::Workload;
use std::fmt::Write;

pub fn generate(
    w: &Workload,
    block: usize,
    pixels: usize,
    tile: usize,
    input_zero: i32,
    stage_weights: bool,
) -> Result<String, String> {
    w.validate()?;
    if w.operator == "depthwise"
        || w.input_offset != 0
        || !(-128..=127).contains(&input_zero)
        || ![1, 2, 4, 8].contains(&block)
        || ![1, 2, 4].contains(&pixels)
        || ![1, 2, 4, 8, 16, 32].contains(&tile)
        || !tile.is_multiple_of(pixels)
        || !w.output_channels.is_multiple_of(block)
    {
        return Err(
            "tiled convolution requires folded bias, packed complete blocks and valid tiles".into(),
        );
    }
    let (ic, oc, k) = (w.input_channels, w.output_channels, w.kernel);
    let reduction = k * k * ic;
    let spatial = w.oh() * w.ow();
    let scratch = tile * reduction + if stage_weights { block * reduction } else { 0 };
    if scratch > 12288 || scratch > w.sram_budget {
        return Err("tile exceeds explicit 12 KiB scratch ceiling or SRAM budget".into());
    }
    let mut s = format!("#![no_std]\n// Scratch: {scratch} bytes on stack, plus compiler spills.\n// Weights [OC/B, K*K*IC, B]; bias includes original input_offset*sum(weights).\n{}\n", include_str!("quant_fast.rs").replace("requantize_nonpositive", "requantize"));
    writeln!(s,"#[inline(never)] pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {{").unwrap();
    writeln!(
        s,
        "assert!(input.len()>={} && weights.len()>={} && output.len()>={});",
        w.input_len(),
        w.weights_len(),
        w.output_len()
    )
    .unwrap();
    writeln!(
        s,
        "assert!(bias.len()>={oc} && mult.len()>={oc} && shift.len()>={oc});"
    )
    .unwrap();
    writeln!(s,"assert!(mult[..{oc}].iter().all(|&m|m>=0) && shift[..{oc}].iter().all(|&q|(-30..=0).contains(&q)));").unwrap();
    writeln!(s, "let mut patch=[0i8;{}];", tile * reduction).unwrap();
    if stage_weights {
        writeln!(s, "let mut local_weights=[0i8;{}];", block * reduction).unwrap();
    }
    writeln!(s,"for base in (0..{spatial}).step_by({tile}) {{ let count=core::cmp::min({tile},{spatial}-base);").unwrap();
    writeln!(s,"for (p,dst) in patch[..count*{reduction}].chunks_exact_mut({reduction}).enumerate() {{ let y=(base+p)/{};let x=(base+p)%{};",w.ow(),w.ow()).unwrap();
    writeln!(s,"for ky in 0..{k} {{for kx in 0..{k} {{let iy=(y*{}+ky) as isize-{};let ix=(x*{}+kx) as isize-{};let target=&mut dst[(ky*{k}+kx)*{ic}..(ky*{k}+kx+1)*{ic}];",w.stride,w.padding,w.stride,w.padding).unwrap();
    writeln!(s,"if iy>=0 && iy<{} && ix>=0 && ix<{} {{let q=(iy as usize*{}+ix as usize)*{ic};target.copy_from_slice(&input[q..q+{ic}]);}} else {{target.fill({input_zero});}} }} }} }}",w.height,w.width,w.width).unwrap();
    writeln!(s,"for c in (0..{oc}).step_by({block}) {{let wb=&weights[c*{reduction}..(c+{block})*{reduction}];").unwrap();
    if stage_weights {
        s.push_str("local_weights.copy_from_slice(wb);let wb=&local_weights[..];\n");
    }
    // Main microtile and scalar tail share arithmetic and parameter layout.
    for (range, p) in [
        (format!("0..count/{pixels}*{pixels}"), pixels),
        (format!("count/{pixels}*{pixels}..count"), 1),
    ] {
        writeln!(s, "for p in ({range}).step_by({p}) {{").unwrap();
        for j in 0..p {
            writeln!(
                s,
                "let v{j}=&patch[(p+{j})*{reduction}..(p+{j}+1)*{reduction}];"
            )
            .unwrap();
        }
        for b in 0..block {
            for j in 0..p {
                writeln!(s, "let mut a{j}_{b}=bias[c+{b}];").unwrap();
            }
        }
        let mut iter = "v0.iter()".to_string();
        let mut pattern = "&v0".to_string();
        for j in 1..p {
            iter = format!("{iter}.zip(v{j}.iter())");
            pattern = format!("({pattern},&v{j})");
        }
        writeln!(
            s,
            "for ({pattern},ws) in {iter}.zip(wb.chunks_exact({block})) {{"
        )
        .unwrap();
        for j in 0..p {
            writeln!(s, "let v{j}=v{j} as i32;").unwrap();
        }
        for b in 0..block {
            writeln!(s, "let w{b}=ws[{b}] as i32;").unwrap();
            for j in 0..p {
                writeln!(s, "a{j}_{b}+=v{j}*w{b};").unwrap();
            }
        }
        s.push_str("}\n");
        for b in 0..block {
            for j in 0..p {
                writeln!(s,"output[(base+p+{j})*{oc}+c+{b}]=(requantize(a{j}_{b},mult[c+{b}],shift[c+{b}])+{}).clamp({},{}) as i8;",w.output_offset,w.activation_min,w.activation_max).unwrap();
            }
        }
        s.push_str("}\n");
    }
    s.push_str("}}}\n/// # Safety\n/// Pointers must have the generated lengths, not alias, and use the packed weights and folded bias.\n#[no_mangle]pub unsafe extern \"C\" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8){\n");
    writeln!(s,"run(core::slice::from_raw_parts(i,{}),core::slice::from_raw_parts(w,{}),core::slice::from_raw_parts(b,{oc}),core::slice::from_raw_parts(m,{oc}),core::slice::from_raw_parts(s,{oc}),core::slice::from_raw_parts_mut(o,{}));}}",w.input_len(),w.weights_len(),w.output_len()).unwrap();
    Ok(s)
}
