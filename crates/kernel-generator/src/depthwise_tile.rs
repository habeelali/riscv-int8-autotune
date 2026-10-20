//! Spatial/channel tiles reuse depthwise weights across adjacent output pixels.
use crate::Workload;
use std::fmt::Write;

pub fn generate(w: &Workload, block: usize, pixels: usize, unroll: bool) -> Result<String, String> {
    w.validate()?;
    if w.operator != "depthwise"
        || w.kernel != 3
        || w.padding != 0
        || w.input_offset != 0
        || ![1, 2, 4, 8].contains(&block)
        || ![1, 2, 4].contains(&pixels)
        || !w.input_channels.is_multiple_of(block)
    {
        return Err(
            "depthwise tiles require explicit padding, folded bias and complete channel blocks"
                .into(),
        );
    }
    let ic = w.input_channels;
    let oh = w.oh();
    let ow = w.ow();
    let width = w.width;
    let stride = w.stride;
    let pixels = if ow.is_multiple_of(pixels) { pixels } else { 1 };
    let mut s = format!(
        "#![no_std]\n// Spatially tiled depthwise, explicit padding. Scratch: 0 bytes.\n{}\n",
        include_str!("quant_fast.rs").replace("requantize_nonpositive", "requantize")
    );
    writeln!(s,"#[inline(never)] pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {{").unwrap();
    writeln!(
        s,
        "assert!(input.len()>={} && weights.len()>={} && output.len()>={});",
        w.input_len(),
        9 * ic,
        w.output_len()
    )
    .unwrap();
    writeln!(
        s,
        "assert!(bias.len()>={ic} && mult.len()>={ic} && shift.len()>={ic});"
    )
    .unwrap();
    writeln!(s,"assert!(mult[..{ic}].iter().all(|&m|m>=0) && shift[..{ic}].iter().all(|&q|(-30..=0).contains(&q)));").unwrap();
    writeln!(s,"for y in 0..{oh} {{ for x in (0..{ow}).step_by({pixels}) {{ for c in (0..{ic}).step_by({block}) {{").unwrap();
    for b in 0..block {
        for p in 0..pixels {
            writeln!(s, "let mut a{p}_{b}=bias[c+{b}];").unwrap();
        }
    }
    let taps: Vec<_> = if unroll {
        (0..3)
            .flat_map(|y| (0..3).map(move |x| (y.to_string(), x.to_string())))
            .collect()
    } else {
        s.push_str("for ky in 0..3 { for kx in 0..3 {\n");
        vec![("ky".into(), "kx".into())]
    };
    for (ky, kx) in taps {
        s.push_str("{\n");
        writeln!(
            s,
            "let wp=({ky}*3+{kx})*{ic}+c; let ws=&weights[wp..wp+{block}];"
        )
        .unwrap();
        for p in 0..pixels {
            writeln!(s,"let ip{p}=((y*{stride}+{ky})*{width}+(x+{p})*{stride}+{kx})*{ic}+c; let v{p}=&input[ip{p}..ip{p}+{block}];").unwrap();
        }
        for b in 0..block {
            writeln!(s, "let wb=ws[{b}] as i32;").unwrap();
            for p in 0..pixels {
                writeln!(s, "a{p}_{b}+=v{p}[{b}] as i32*wb;").unwrap();
            }
        }
        s.push_str("}\n");
    }
    if !unroll {
        s.push_str("} }\n");
    }
    for b in 0..block {
        for p in 0..pixels {
            writeln!(s,"output[(y*{ow}+x+{p})*{ic}+c+{b}]=(requantize(a{p}_{b},mult[c+{b}],shift[c+{b}])+{}).clamp({},{}) as i8;",w.output_offset,w.activation_min,w.activation_max).unwrap();
        }
    }
    s.push_str("} } } }\n/// # Safety\n/// Tensors must match the explicit-padding and folded-bias model contract.\n#[no_mangle]\npub unsafe extern \"C\" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {\n");
    writeln!(s,"run(core::slice::from_raw_parts(i,{}),core::slice::from_raw_parts(w,{}),core::slice::from_raw_parts(b,{ic}),core::slice::from_raw_parts(m,{ic}),core::slice::from_raw_parts(s,{ic}),core::slice::from_raw_parts_mut(o,{})); }}",w.input_len(),9*ic,w.output_len()).unwrap();
    Ok(s)
}
