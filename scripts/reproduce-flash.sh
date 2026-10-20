#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
source scripts/env.sh
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
python3 -m unittest discover -s tests -v
cargo run --locked -p autotuner -- tune-flash
bash scripts/build-plugin.sh
python3 scripts/evaluate_flash.py
python3 scripts/report_flash.py
