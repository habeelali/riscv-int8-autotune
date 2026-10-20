//! Code sharing for this generator's model modules; never shares tensor data.
use std::collections::BTreeMap;

/// Compact the controlled `model_sources` output. Rejects unfamiliar framing.
/// Equality is checked on emitted code, including bounds, offsets and clamps;
/// only comments and the exported entry-point name are ignored.
pub fn compact_model(
    source: &str,
    deduplicate: bool,
    outline_quant: bool,
) -> Result<String, String> {
    let (modules, panic) = source
        .split_once("\n#[panic_handler]")
        .ok_or("generated panic boundary missing")?;
    if !modules.starts_with("#![no_std]\nmod layer_") {
        return Err("unrecognized generated model framing".into());
    }
    let mut result = String::from("#![no_std]\n");
    let mut canonical = BTreeMap::<String, usize>::new();
    let mut shared_quant = None;
    let mut seen = 0;
    for part in modules.split("\nmod layer_").skip(1) {
        let (index, body) = part.split_once(" {\n").ok_or("module header missing")?;
        let index: usize = index.parse().map_err(|_| "invalid layer number")?;
        let body = body
            .trim_end()
            .strip_suffix("\n}")
            .ok_or("module footer missing")?;
        let normalized = body
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n")
            .replace(&format!("fn layer_{index}("), "fn kernel(");
        let duplicate = if deduplicate {
            canonical.get(&normalized).copied()
        } else {
            None
        };
        let mut emitted = if let Some(first) = duplicate {
            let wrapper = body
                .split_once("/// # Safety\n")
                .ok_or("entry-point contract missing")?
                .1;
            format!(
                "/// # Safety\n{}",
                wrapper.replace("\nrun(", &format!("\nsuper::layer_{first}::run("))
            )
        } else {
            canonical.insert(normalized, index);
            body.to_string()
        };
        if outline_quant && duplicate.is_none() {
            let start = emitted
                .find("#[inline(always)]")
                .ok_or("quantizer missing")?;
            let end = emitted
                .find("/// All buffers are caller owned.")
                .ok_or("run boundary missing")?;
            let quant = emitted[start..end].to_string();
            if let Some(ref previous) = shared_quant {
                if previous != &quant {
                    return Err("quantization implementations differ".into());
                }
            } else {
                shared_quant = Some(quant);
            }
            emitted.replace_range(start..end, "use super::requantize;\n");
        }
        result.push_str(&format!("mod layer_{index} {{\n{emitted}\n}}\n"));
        seen += 1;
    }
    if seen == 0 {
        return Err("no generated layers".into());
    }
    if let Some(quant) = shared_quant {
        result.push_str(&quant.replace("#[inline(always)]", "#[inline(never)]"));
    }
    result.push_str("#[panic_handler]");
    result.push_str(panic);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{generate, grid, Workload};

    #[test]
    fn shares_only_identical_generated_code_and_preserves_entry_points() {
        let w = Workload::parse(include_str!("../../../workloads/conv_small.toml")).unwrap();
        let original = generate(&w, grid(&w)[0]).unwrap().replace("#![no_std]", "");
        let a = original.replace("fn kernel(", "fn layer_0(");
        let b = original.replace("fn kernel(", "fn layer_1(");
        let c = original
            .replace("fn kernel(", "fn layer_2(")
            .replace(".clamp(", ".max(-100).clamp(");
        let source = format!("#![no_std]\nmod layer_0 {{\n{a}\n}}\nmod layer_1 {{\n{b}\n}}\nmod layer_2 {{\n{c}\n}}\n#[panic_handler] fn panic() {{}}\n");
        let compact = compact_model(&source, true, false).unwrap();
        assert_eq!(compact.matches("pub fn run(").count(), 2);
        assert!(compact.contains("super::layer_0::run("));
        assert!(compact.contains("fn layer_1("));
        assert!(compact.contains("fn layer_2("));
        assert!(compact_model("arbitrary source", true, false).is_err());
    }
}
