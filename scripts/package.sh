#!/usr/bin/env bash
# Source, measurements and executable evidence; no tool installation or Git metadata.
set -euo pipefail
project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
project_name="${project_dir##*/}"
archive_path="${1:-${project_dir}.tar.gz}"
python3 "$project_dir/scripts/prune-build-cache.py"
tar -czf "$archive_path" -C "$(dirname "$project_dir")" \
 --exclude="$project_name/.git" --exclude="$project_name/baselines/esp-nn/.git" \
 --exclude="$project_name/target" --exclude="$project_name/.tools" --exclude="$project_name/.venv" \
 --exclude='.git' --exclude='__pycache__' --exclude='*.pyc' --exclude='count-plugin.so' "$project_name"
sha256sum "$archive_path" > "${archive_path}.sha256"
ls -lh "$archive_path"
