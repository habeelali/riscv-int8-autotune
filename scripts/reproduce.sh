#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
source scripts/env.sh
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
python3 -m unittest discover -s tests -v
cargo run --locked -p autotuner -- benchmark-baseline
cargo run --locked -p autotuner -- benchmark-rust
cargo run --locked -p autotuner -- benchmark-competitors
python3 scripts/replay_previous.py
cargo run --locked -p autotuner -- tune all
cargo run --locked -p autotuner -- regression
python3 scripts/regression_muriscv.py
scripts/build-plugin.sh
python3 scripts/validate_counter.py
cargo run --locked -p autotuner -- export
cargo run --locked -p autotuner -- compare
python3 scripts/visualize_before_after.py
python3 scripts/update_readme_results.py
python3 scripts/prune-build-cache.py
python3 scripts/write_validation.py
