#!/usr/bin/env bash
# Optional pretrained-model suite; install requirements-model.txt first.
set -euo pipefail
cd "$(dirname "$0")/.."
source scripts/env.sh
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
python3 -m unittest discover -s tests -v
cargo run --locked -p autotuner -- benchmark-model
bash scripts/build-plugin.sh
python3 scripts/validate_model_counter.py
python3 scripts/report_model.py
