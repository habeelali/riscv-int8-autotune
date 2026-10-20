//! Emit a model with explicit offline-parameter transforms in its manifest.
use kernel_generator::{
    flash::compact_model,
    generate,
    packed::{packed_weights, pointwise_chunks_ordered},
    Config, Workload,
};
use serde_json::Value;
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    let manifest: Value = serde_json::from_str(&fs::read_to_string(&args[1])?)?;
    let mut source = String::from("#![no_std]\n");
    for layer in manifest["layers"].as_array().ok_or("missing layers")? {
        if layer.get("workload").is_none() {
            continue;
        }
        let index = layer["index"].as_u64().ok_or("missing layer number")?;
        let w: Workload = serde_json::from_value(layer["workload"].clone())?;
        let cfg: Config = serde_json::from_value(layer["config"].clone())?;
        let mut code = if let Some(tile) = layer["cache_tile"].as_u64() {
            kernel_generator::tiled::generate(
                &w,
                cfg.block,
                cfg.pixels,
                tile as usize,
                layer["original_input_zero"]
                    .as_i64()
                    .ok_or("missing padding zero point")? as i32,
                layer["stage_weights"] == true,
            )
            .map_err(std::io::Error::other)?
        } else if layer["dw_tile"] == true {
            kernel_generator::depthwise_tile::generate(
                &w,
                cfg.block,
                cfg.pixels,
                layer["dw_unroll"] == true,
            )
            .map_err(std::io::Error::other)?
        } else if layer["chunks"] == true {
            pointwise_chunks_ordered(
                &w,
                cfg.block,
                cfg.pixels,
                layer["cache_channel_outer"] == true,
            )
            .map_err(std::io::Error::other)?
        } else {
            generate(&w, cfg).map_err(std::io::Error::other)?
        };
        if layer["packed"] == true && layer["chunks"] != true && layer.get("cache_tile").is_none() {
            code = packed_weights(&code, &w, cfg.block).map_err(std::io::Error::other)?;
        }
        if layer["bounded_quant"] == true {
            let old = include_str!("../src/quant_fast.rs")
                .replace("requantize_nonpositive", "requantize");
            if !code.contains(&old) {
                return Err("default quantizer missing".into());
            }
            code = code.replace(
                &old,
                include_str!("../src/quant_bounded.rs")
                    .split("#[cfg(test)]")
                    .next()
                    .ok_or("quantizer missing")?,
            );
            code = code.replace("(-30..=0).contains(&q)", "(-30..=-1).contains(&q)");
        }
        if layer["prevalidated"] == true {
            if layer["bounded_quant"] != true {
                return Err("host quantization proof required".into());
            }
            let lines: Vec<_> = code
                .lines()
                .filter(|l| l.contains("assert!(mult[.."))
                .collect();
            if lines.len() != 1 {
                return Err("quantization validation boundary missing".into());
            }
            code = code.replace(
                lines[0],
                "// Quantization ranges and accumulator bound are proven by the model packer.",
            );
            code=code.replace("/// # Safety","/// # Safety\n/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.");
        }
        code = code
            .replace("#![no_std]", "")
            .replace("fn kernel(", &format!("fn layer_{index}("));
        source.push_str(&format!("mod layer_{index} {{\n{code}\n}}\n"));
    }
    source.push_str("#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { loop { core::hint::spin_loop(); } }\n");
    let source = compact_model(&source, true, false).map_err(std::io::Error::other)?;
    fs::write(&args[2], source)?;
    Ok(())
}
