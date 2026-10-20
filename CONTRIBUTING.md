# Contributing

Start with [the tool workflow](docs/tool.md), [reproducibility](docs/reproducibility.md)
and the [research scope](docs/research-summary.md).

For source changes, run:

```bash
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
python3 -m unittest discover -s tests -v
```

For CLI or generator changes, also build the CLI and run
`python3 scripts/test_tool_e2e.py` after setting up the cross tools. Changes to
cache tiling need `python3 scripts/test_cache_tiles.py`. Changes to physical-log
validation need `bash scripts/audit-v5.sh` and the negative audit controls.

Performance changes require a correct, saved baseline first, identical arithmetic
and tensor fixtures, the actual compiler/ISA configuration, and all candidate
measurements including regressions. Compare QEMU instructions separately from
physical timings. Record RAM, scratch and code-size costs. Do not treat fewer
instructions as proof of faster hardware inference.

Keep raw physical returns immutable. Derived reports may be regenerated. Do not
remove failed captures, silently change tensors, relax audit checks to accept a
run, or claim language-level or first-ever novelty from these experiments.
Third-party code must retain its license and pin. Do not commit local toolchains,
build caches, serial credentials or device access tokens. Use a separate job
output directory for experiments.
