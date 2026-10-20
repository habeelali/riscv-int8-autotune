#!/usr/bin/env bash
# Debian 13 x86_64, no sudo required. Leaves the OS package database untouched.
set -euo pipefail
project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
tools_dir="${RISCV_TOOLS_PREFIX:-$project_dir/.tools}"
apt_dir="$tools_dir/apt"
mkdir -p "$apt_dir/lists/partial" "$apt_dir/cache/archives/partial" "$apt_dir/debs"
cat > "$apt_dir/sources.list" <<'SOURCES'
deb [check-valid-until=no] https://snapshot.debian.org/archive/debian/20260925T181338Z trixie main
deb [check-valid-until=no] https://snapshot.debian.org/archive/debian-security/20260925T181338Z trixie-security main
SOURCES
apt_options=(-o "Dir::State::lists=$apt_dir/lists" -o "Dir::Cache=$apt_dir/cache" -o "Dir::Etc::sourcelist=$apt_dir/sources.list" -o Dir::Etc::sourceparts=-)
apt-get "${apt_options[@]}" update -qq
(cd "$apt_dir/debs" && apt-get "${apt_options[@]}" download \
 'qemu-system-riscv=1:10.0.13+ds-0+deb13u1' 'qemu-system-common=1:10.0.13+ds-0+deb13u1' 'qemu-system-data=1:10.0.13+ds-0+deb13u1' \
 'gcc-riscv64-unknown-elf=14.2.0+19' 'binutils-riscv64-unknown-elf=2.44-3+7+b1' \
 libfdt1 libslirp0 liburing2 libcapstone5 libaio1t64 libpmem1 libvdeplug2t64 libndctl6 libdaxctl1 libgio-2.0-dev)
for deb in "$apt_dir/debs/"*.deb; do dpkg-deb -x "$deb" "$tools_dir"; done
if ! command -v rustup >/dev/null && [ ! -x "$HOME/.cargo/bin/rustup" ]; then
 curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o "$tools_dir/rustup-init.sh"
 sh "$tools_dir/rustup-init.sh" -y --profile minimal --default-toolchain 1.90.0
fi
export PATH="$HOME/.cargo/bin:$PATH"
rustup toolchain install 1.90.0 --profile minimal --component rustfmt,clippy --target riscv32imc-unknown-none-elf
export RISCV_TOOLS_PREFIX="$tools_dir"
# shellcheck source=env.sh
source "$project_dir/scripts/env.sh"
qemu-system-riscv32 --version
riscv64-unknown-elf-gcc --version
cargo --version
(cd "$project_dir" && cargo fetch --locked)
echo "Setup complete. Run: source scripts/env.sh"
