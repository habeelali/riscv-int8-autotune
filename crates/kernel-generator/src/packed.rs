//! Scalar output-channel-blocked weight indexing; no target SIMD instructions.
use crate::Workload;

/// Replace the known generator's OHWI loads with [OC/B, reduction, B] loads.
/// The caller must pack weights in the matching layout; byte count is unchanged.
pub fn packed_weights(source: &str, w: &Workload, block: usize) -> Result<String, String> {
    if ![1, 2, 4, 8, 16].contains(&block)
        || w.operator == "depthwise"
        || !w.output_channels.is_multiple_of(block)
    {
        return Err("packed loads require complete ordinary-convolution blocks".into());
    }
    let n = w.kernel * w.kernel * w.input_channels;
    let mut result = String::new();
    let mut rest = source;
    let mut replaced = 0;
    while let Some(position) = rest.find("weights[") {
        result.push_str(&rest[..position]);
        rest = &rest[position + "weights[".len()..];
        let end = rest.find(']').ok_or("weight subscript is not closed")?;
        let index = &rest[..end];
        let mut mapped = None;
        for b in 0..block {
            if let Some(reduction) = index.strip_prefix(&format!("(c+{b})*{n}+")) {
                mapped = Some(format!(
                    "(c/{block})*{}+({reduction})*{block}+{b}",
                    n * block
                ));
            }
            if let Some(reduction) = index.strip_prefix(&format!("((c+{b})*{n}+wp)+")) {
                mapped = Some(format!(
                    "(c/{block})*{}+(wp+{reduction})*{block}+{b}",
                    n * block
                ));
            }
        }
        let mapped =
            mapped.ok_or_else(|| format!("unrecognized generated weight index: {index}"))?;
        result.push_str(&format!("weights[{mapped}]"));
        rest = &rest[end + 1..];
        replaced += 1;
    }
    if replaced == 0 {
        return Err("no weight loads found".into());
    }
    result.push_str(rest);
    Ok(result)
}

/// Flat spatial GEMM with safe fixed-size chunks. This consumes offline-packed
/// weights and zero-point-folded bias; no input copy or scratch is introduced.
pub fn pointwise_chunks(w: &Workload, block: usize, pixels: usize) -> Result<String, String> {
    pointwise_chunks_ordered(w, block, pixels, false)
}

/// Channel-first traversal reuses a small weight block across all pixels.
/// Arithmetic and packed layout are identical to `pointwise_chunks`.
pub fn pointwise_chunks_ordered(
    w: &Workload,
    block: usize,
    pixels: usize,
    channel_outer: bool,
) -> Result<String, String> {
    w.validate()?;
    if w.operator != "conv"
        || w.kernel != 1
        || w.stride != 1
        || w.padding != 0
        || w.input_offset != 0
        || ![1, 2, 4, 8].contains(&block)
        || ![1, 2, 4].contains(&pixels)
        || !w.output_channels.is_multiple_of(block)
    {
        return Err("chunk GEMM requires folded, packed, complete pointwise channel blocks".into());
    }
    use std::fmt::Write;
    let ic = w.input_channels;
    let oc = w.output_channels;
    let spatial = w.height * w.width;
    let mut s = format!(
        "#![no_std]\n// Packed safe-chunk pointwise GEMM. Scratch: 0 bytes.\n{}\n",
        include_str!("quant_fast.rs").replace("requantize_nonpositive", "requantize")
    );
    writeln!(s,"#[inline(never)] pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {{").unwrap();
    writeln!(
        s,
        "assert!(input.len()>={} && weights.len()>={} && output.len()>={});",
        spatial * ic,
        ic * oc,
        spatial * oc
    )
    .unwrap();
    writeln!(
        s,
        "assert!(bias.len()>={oc} && mult.len()>={oc} && shift.len()>={oc});"
    )
    .unwrap();
    writeln!(s,"assert!(mult[..{oc}].iter().all(|&m|m>=0) && shift[..{oc}].iter().all(|&q|(-30..=0).contains(&q)));").unwrap();
    if channel_outer {
        writeln!(
            s,
            "for c in (0..{oc}).step_by({block}) {{ let wb=&weights[c*{ic}..(c+{block})*{ic}];"
        )
        .unwrap();
    }
    for (start, end, p) in [
        (0, spatial / pixels * pixels, pixels),
        (spatial / pixels * pixels, spatial, 1),
    ] {
        if start == end {
            continue;
        }
        writeln!(s,"for pixel in ({start}..{end}).step_by({p}) {{ let tile=&input[pixel*{ic}..(pixel+{p})*{ic}]; let out=&mut output[pixel*{oc}..(pixel+{p})*{oc}];").unwrap();
        for j in 0..p {
            writeln!(s, "let v{j}=&tile[{}..{}];", j * ic, (j + 1) * ic).unwrap();
        }
        if !channel_outer {
            writeln!(
                s,
                "for c in (0..{oc}).step_by({block}) {{ let wb=&weights[c*{ic}..(c+{block})*{ic}];"
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
            pattern = format!("({pattern}, &v{j})");
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
                writeln!(s,"out[{}+c+{b}]=(requantize(a{j}_{b},mult[c+{b}],shift[c+{b}])+{}).clamp({},{}) as i8;",j*oc,w.output_offset,w.activation_min,w.activation_max).unwrap();
            }
        }
        s.push_str("}\n");
        if !channel_outer {
            s.push_str("}\n");
        }
    }
    if channel_outer {
        s.push_str("}\n");
    }
    s.push_str("}\n/// # Safety\n/// Tensor lengths, non-aliasing and packed parameter layout must match the generated manifest.\n#[no_mangle]\npub unsafe extern \"C\" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {\n");
    writeln!(s,"run(core::slice::from_raw_parts(i,{}),core::slice::from_raw_parts(w,{}),core::slice::from_raw_parts(b,{oc}),core::slice::from_raw_parts(m,{oc}),core::slice::from_raw_parts(s,{oc}),core::slice::from_raw_parts_mut(o,{})); }}",spatial*ic,ic*oc,spatial*oc).unwrap();
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{generate, grid};

    #[test]
    fn maps_generated_indices_and_rejects_partial_blocks() {
        let w = Workload::parse(include_str!("../../../workloads/pointwise.toml")).unwrap();
        let cfg = grid(&w)
            .into_iter()
            .find(|c| c.block == 4 && c.pixels == 2)
            .unwrap();
        let packed = packed_weights(&generate(&w, cfg).unwrap(), &w, 4).unwrap();
        assert!(packed.contains("(c/4)*128+"));
        assert!(packed_weights("weights[unknown]", &w, 4).is_err());
        let mut odd = w;
        odd.output_channels = 3;
        assert!(packed_weights("weights[unknown]", &odd, 4).is_err());
    }
}
