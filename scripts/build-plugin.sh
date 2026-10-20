#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
source scripts/env.sh
if pkg-config --exists glib-2.0 2>/dev/null; then
 read -r -a glib_flags <<< "$(pkg-config --cflags glib-2.0)"
else
 prefix="${RISCV_TOOLS_PREFIX:-$PWD/.tools}"
 if [ ! -d "$prefix" ] && [ -d /workspace/tools ]; then prefix=/workspace/tools; fi
 glib_flags=(-I"$prefix/usr/include/glib-2.0" -I"$prefix/usr/lib/x86_64-linux-gnu/glib-2.0/include")
fi
cc -O2 -fPIC -shared "${glib_flags[@]}" qemu/count-plugin.c -o qemu/count-plugin.so
