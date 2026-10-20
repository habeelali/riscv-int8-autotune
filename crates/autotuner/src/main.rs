use kernel_generator::{generate, grid, Workload};
use serde_json::{json, Value};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    time::Instant,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn root() -> PathBuf {
    if let Some(path) = env::var_os("RISCV_INT8_ROOT") {
        return PathBuf::from(path);
    }
    if let Ok(cwd) = env::current_dir() {
        for path in cwd.ancestors() {
            if path.join("scripts/runner.py").is_file()
                && path.join("crates/kernel-generator/Cargo.toml").is_file()
            {
                return path.to_path_buf();
            }
        }
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
}
fn backend(script: &str, args: &[&str]) -> Result<()> {
    let status = Command::new("python3")
        .arg(root().join("scripts").join(script))
        .args(args)
        .current_dir(root())
        .status()?;
    if !status.success() {
        return Err(format!("{script} failed: {status}").into());
    }
    Ok(())
}
fn read(path: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}
fn write(path: impl AsRef<Path>, v: &Value) -> Result<()> {
    fs::write(path, serde_json::to_string_pretty(v)? + "\n")?;
    Ok(())
}
fn metric(v: &Value, k: &str) -> u64 {
    v[k].as_u64().unwrap()
}
fn eligible(v: &Value, w: &Workload) -> bool {
    v["correct"] == true
        && metric(v, "flash_bytes") <= w.flash_budget as u64
        && metric(v, "static_ram_bytes") + metric(v, "reserved_stack_bytes") <= w.sram_budget as u64
}
fn choose(rows: &[Value], w: &Workload, objective: &str) -> Result<(Value, Vec<Value>)> {
    let mut valid: Vec<_> = rows
        .iter()
        .filter(|r| r.get("error").is_none() && eligible(r, w))
        .cloned()
        .collect();
    valid.sort_by_key(|v| {
        if objective == "min-flash" {
            (metric(v, "flash_bytes"), metric(v, "instructions"))
        } else {
            (metric(v, "instructions"), metric(v, "flash_bytes"))
        }
    });
    let best = valid
        .first()
        .ok_or("no correct candidate meets resource constraints")?
        .clone();
    let pareto = valid
        .iter()
        .filter(|v| {
            !valid.iter().any(|u| {
                metric(u, "instructions") <= metric(v, "instructions")
                    && metric(u, "flash_bytes") <= metric(v, "flash_bytes")
                    && (metric(u, "instructions") < metric(v, "instructions")
                        || metric(u, "flash_bytes") < metric(v, "flash_bytes"))
            })
        })
        .cloned()
        .collect();
    Ok((best, pareto))
}
fn tune(path: &Path, objective: &str) -> Result<()> {
    if !["min-instructions", "min-flash", "budget", "pareto"].contains(&objective) {
        return Err("objective must be min-instructions, min-flash, budget, or pareto".into());
    }
    if !root().join("results/generic-milestone.json").exists() {
        return Err("run benchmark-baseline then benchmark-rust before tuning".into());
    }
    let w = Workload::parse(&fs::read_to_string(path)?).map_err(std::io::Error::other)?;
    let path = path.canonicalize()?;
    backend(
        "prepare-workload.py",
        &[path.to_str().ok_or("non-UTF8 workload path")?],
    )?;
    let dir = root().join("generated").join(&w.name);
    fs::create_dir_all(&dir)?;
    let result = root()
        .join("results")
        .join(format!("tuning-{}.json", w.name));
    let started = Instant::now();
    let mut rows = Vec::new();
    for cfg in grid(&w) {
        let name = cfg.name();
        let source = dir.join(format!("{name}.rs"));
        let measured = dir.join(format!("{name}.json"));
        fs::write(&source, generate(&w, cfg).map_err(std::io::Error::other)?)?;
        let t = Instant::now();
        let out = Command::new("python3")
            .arg(root().join("scripts/runner.py"))
            .arg("one")
            .arg("--workload")
            .arg(&path)
            .args(["--kind", "generated", "--source"])
            .arg(&source)
            .arg("--out")
            .arg(&measured)
            .current_dir(root())
            .output()?;
        fs::write(
            dir.join(format!("{name}.log")),
            [out.stdout.as_slice(), out.stderr.as_slice()].concat(),
        )?;
        let mut row = if out.status.success() {
            read(&measured)?
        } else {
            json!({"error":String::from_utf8_lossy(&out.stderr),"correct":false})
        };
        row["config"] = serde_json::to_value(cfg)?;
        row["variant"] = json!(name);
        row["source"] = json!(source.strip_prefix(root())?.to_str());
        row["search_evaluation_seconds"] = json!(t.elapsed().as_secs_f64());
        println!(
            "{} {}: {}",
            w.name,
            name,
            if row.get("error").is_some() {
                "REJECTED".to_string()
            } else {
                format!(
                    "{} instructions, {} text bytes",
                    row["instructions"], row["text_bytes"]
                )
            }
        );
        rows.push(row);
        write(&result, &json!(rows))?;
    }
    let (best, pareto) = choose(&rows, &w, objective)?;
    let summary = json!({"workload":w.name,"objective":objective,"best":best,"pareto":pareto,"candidates":rows.len(),"search_wall_seconds":started.elapsed().as_secs_f64()});
    write(
        root().join("results").join(format!("best-{}.json", w.name)),
        &summary,
    )?;
    fs::copy(
        root().join(best["source"].as_str().unwrap()),
        dir.join("best.rs"),
    )?;
    println!(
        "Selected {}: {} instructions",
        best["variant"], best["instructions"]
    );
    Ok(())
}
fn workloads() -> Result<Vec<PathBuf>> {
    let mut paths: Vec<_> = fs::read_dir(root().join("workloads"))?
        .map(|p| p.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "toml"))
        .collect();
    paths.sort();
    Ok(paths)
}
fn export(name: Option<&String>, out: Option<&String>) -> Result<()> {
    let paths = if let Some(n) = name {
        vec![root().join("workloads").join(format!("{n}.toml"))]
    } else {
        workloads()?
    };
    let out = out
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("generated/export"));
    fs::create_dir_all(&out)?;
    for path in paths {
        let w = Workload::parse(&fs::read_to_string(path)?).map_err(std::io::Error::other)?;
        let best = read(root().join("results").join(format!("best-{}.json", w.name)))?;
        let source = root().join(
            best["best"]["source"]
                .as_str()
                .ok_or("missing generated source")?,
        );
        let standalone = fs::read_to_string(source)?.replace(
            "#[no_mangle]",
            &format!("#[export_name = \"riscv_int8_{}\"]", w.name),
        );
        fs::write(out.join(format!("{}.rs", w.name)), standalone)?;
        write(out.join(format!("{}.json", w.name)), &best)?;
        println!("Exported {}", w.name);
    }
    Ok(())
}
fn execute() -> Result<()> {
    let a: Vec<_> = env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        None | Some("--help" | "-h" | "help") => {
            println!("riscv-int8 — measured scalar RV32IMC INT8 kernels\n\nUsage:\n  riscv-int8 doctor [--json]\n  riscv-int8 init WORKLOAD.toml [--operator conv|pointwise|depthwise|fc]\n  riscv-int8 inspect WORKLOAD.toml\n  riscv-int8 build WORKLOAD.toml --out DIR [--max-candidates N]\n                   [--objective min-instructions|min-flash|budget|pareto] [--resume]\n\nBuild validates C first, then generic Rust, measures candidates in QEMU,\nand exports a reusable no_std crate, measurements and an HTML report.\nRun from a source checkout, or set RISCV_INT8_ROOT to its directory.\nInstall: cargo install --locked --path crates/autotuner\n\nResearch commands: benchmark-baseline, benchmark-rust, benchmark-model,\nbenchmark-competitors, tune, tune-model, tune-flash, compare, export, regression.\nSource generation only: generate WORKLOAD.toml DIR [MAX_CANDIDATES].");
            return Ok(());
        }
        Some("--version" | "-V") => {
            println!("riscv-int8 {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        Some("doctor" | "init" | "build") => {
            let status = Command::new("python3")
                .arg(root().join("scripts/tool.py"))
                .args(&a)
                .env("RISCV_INT8_BIN", env::current_exe()?)
                .status()?;
            return if status.success() {
                Ok(())
            } else {
                Err(format!("{} failed; see diagnostics above", a[0]).into())
            };
        }
        Some("inspect" | "generate") => {
            let path = a.get(1).ok_or("a workload TOML path is required")?;
            let w = Workload::parse(&fs::read_to_string(path)?).map_err(std::io::Error::other)?;
            let configs = grid(&w);
            if a[0] == "inspect" {
                println!(
                    "{}",
                    serde_json::to_string_pretty(
                        &json!({"spec":w,"input_elements":w.input_len(),"weights_elements":w.weights_len(),"output_elements":w.output_len(),"output_height":w.oh(),"output_width":w.ow(),"search_candidates":configs.len(),"metric":"QEMU executed guest instructions; not hardware cycles"})
                    )?
                );
            } else {
                let out = PathBuf::from(a.get(2).ok_or("generate requires an output directory")?);
                let limit = a
                    .get(3)
                    .map(|v| v.parse::<usize>())
                    .transpose()?
                    .unwrap_or(configs.len());
                if limit == 0 {
                    return Err("candidate limit must be positive".into());
                }
                if out.exists() && fs::read_dir(&out)?.next().is_some() {
                    return Err(
                        "generate output must be empty; existing source will not be overwritten"
                            .into(),
                    );
                }
                fs::create_dir_all(&out)?;
                let mut rows = Vec::new();
                let count = limit.min(configs.len());
                let indices: Vec<_> = if count == 1 {
                    vec![0]
                } else {
                    (0..count)
                        .map(|i| i * (configs.len() - 1) / (count - 1))
                        .collect()
                };
                for index in indices {
                    let cfg = configs[index];
                    let name = cfg.name();
                    fs::write(
                        out.join(format!("{name}.rs")),
                        generate(&w, cfg).map_err(std::io::Error::other)?,
                    )?;
                    rows.push(json!({"variant":name,"config":cfg,"source":format!("{name}.rs")}));
                }
                write(out.join("candidates.json"), &json!(rows))?;
                println!("Generated {} unmeasured candidates", rows.len());
            }
            return Ok(());
        }
        _ => {}
    }
    match a.first().map(String::as_str) {
        Some("benchmark-baseline")=>backend("runner.py",&["baseline"]),
        Some("benchmark-rust")=>backend("benchmark_generic.py",&[]),
        Some("benchmark-model")=>backend("benchmark_model.py",&[]),
        Some("tune-flash")=>backend("tune_flash.py",&[]),
        Some("tune-model")=>backend("tune_algorithm.py",&[]),
        Some("benchmark-competitors")=>{
            backend("validate_upstream_quant.py",&[])?;
            backend("benchmark_muriscv.py",&[])
        },
        Some("tune")=>{let path=a.get(1).ok_or("usage: tune WORKLOAD [OBJECTIVE]")?;let obj=a.get(2).map(String::as_str).unwrap_or("min-instructions");if path=="all" {for p in workloads()?{tune(&p,obj)?;}Ok(())}else{tune(Path::new(path),obj)}},
        Some("compare")=>{
            backend("report.py",&[])?;
            if root().join("results/muriscv-milestone.json").exists(){backend("report_competitors.py",&[])?;}
            Ok(())
        },
        Some("export")=>export(a.get(1),a.get(2)),
        Some("regression")=>backend("regression.py",&[]),
        _=>Err("commands: benchmark-baseline | benchmark-rust | benchmark-model | tune-flash | tune-model | benchmark-competitors | tune WORKLOAD|all [min-instructions|min-flash|budget|pareto] | compare | export [NAME [DIR]] | regression".into())
    }
}
fn main() -> ExitCode {
    match execute() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
