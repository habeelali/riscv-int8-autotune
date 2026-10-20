//! Generate independently compiled layers for the strict person-detector importer.
use kernel_generator::{generate, Config, Workload};
use serde_json::Value;
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    let layers: Value = serde_json::from_str(&fs::read_to_string(&args[1])?)?;
    let selected: Option<Value> = if args[3].ends_with(".json") {
        Some(serde_json::from_str(&fs::read_to_string(&args[3])?)?)
    } else {
        None
    };
    let mut source = String::from("#![no_std]\n");
    for layer in layers["layers"].as_array().ok_or("layers missing")? {
        if layer.get("workload").is_none() {
            continue;
        }
        let w: Workload = serde_json::from_value(layer["workload"].clone())?;
        let index = layer["index"].as_u64().ok_or("layer index missing")?;
        let schedule: usize = if let Some(ref selected) = selected {
            selected["schedules"][index.to_string()]
                .as_u64()
                .ok_or("selected layer schedule missing")? as usize
        } else {
            args[3].parse()?
        };
        if schedule > 3 {
            return Err("schedule must be 0..3".into());
        }
        let mut cfg = Config {
            block: 4,
            unroll: 1,
            spatial_unroll: w.operator == "depthwise",
            im2col: false,
            interior: false,
            pixels: 1,
            channel_outer: false,
            fast_quant: true,
        };
        if w.operator == "conv" {
            cfg.block = if schedule == 1 { 2 } else { 4 };
            cfg.pixels = if w.ow() >= 2 { 2 } else { 1 };
            cfg.im2col = w.kernel > 1;
            if schedule == 2 && w.ow() >= 4 {
                cfg.pixels = 4;
                cfg.block = 2;
            }
            if schedule == 3 {
                cfg.pixels = 1;
                cfg.im2col = false;
                cfg.block = 8;
            }
        } else if schedule == 1 {
            cfg.block = 1;
            cfg.channel_outer = true;
        } else if schedule == 2 {
            cfg.block = 8;
        } else if schedule == 3 {
            cfg.block = 2;
        }
        let generated = generate(&w, cfg).map_err(std::io::Error::other)?;
        source.push_str(&format!("mod layer_{index} {{\n"));
        source.push_str(
            &generated
                .replace("#![no_std]", "")
                .replace("fn kernel(", &format!("fn layer_{index}(")),
        );
        source.push_str("\n}\n");
    }
    source.push_str("#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { loop { core::hint::spin_loop(); } }\n");
    fs::write(&args[2], source)?;
    Ok(())
}
