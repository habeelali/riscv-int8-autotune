use kernel_generator::flash::compact_model;
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    let mode = args.get(3).ok_or("usage: flash_model INPUT OUTPUT MODE")?;
    if !["dedup", "outline", "dedup-outline"].contains(&mode.as_str()) {
        return Err("mode must be dedup, outline or dedup-outline".into());
    }
    let input = fs::read_to_string(&args[1])?;
    let output = compact_model(&input, mode.contains("dedup"), mode.contains("outline"))
        .map_err(std::io::Error::other)?;
    fs::write(&args[2], output)?;
    Ok(())
}
