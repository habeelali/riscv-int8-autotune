#!/usr/bin/env bash
project_tools="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/.tools"
if [ -n "${RISCV_TOOLS_PREFIX:-}" ]; then project_tools="$RISCV_TOOLS_PREFIX";
elif [ ! -d "$project_tools" ] && [ -d /workspace/tools ]; then project_tools=/workspace/tools; fi
if [ -d "$project_tools/usr/bin" ]; then
 export PATH="$project_tools/usr/bin:$PATH"
 export LD_LIBRARY_PATH="$project_tools/usr/lib/x86_64-linux-gnu${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
fi
export PATH="$HOME/.cargo/bin:$PATH"
unset project_tools
