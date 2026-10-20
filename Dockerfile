FROM debian:trixie-slim
# Snapshot fixes GCC, QEMU and apt dependencies; Rust and Cargo.lock are pinned.
RUN printf '%s\n' 'deb [check-valid-until=no] https://snapshot.debian.org/archive/debian/20260925T181338Z trixie main' 'deb [check-valid-until=no] https://snapshot.debian.org/archive/debian-security/20260925T181338Z trixie-security main' > /etc/apt/sources.list \
 && rm -f /etc/apt/sources.list.d/debian.sources
# Bootstrap TLS using the distribution image's normal apt HTTP transport.
RUN sed -i 's|https://snapshot|http://snapshot|g' /etc/apt/sources.list \
 && apt-get update && apt-get install -y --no-install-recommends ca-certificates curl git build-essential pkg-config libglib2.0-dev python3 python3-matplotlib \
 'qemu-system-riscv=1:10.0.13+ds-0+deb13u1' 'gcc-riscv64-unknown-elf=14.2.0+19' 'binutils-riscv64-unknown-elf=2.44-3+7+b1' \
 && rm -rf /var/lib/apt/lists/*
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/rustup.sh \
 && sh /tmp/rustup.sh -y --profile minimal --default-toolchain 1.90.0
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup component add rustfmt clippy && rustup target add riscv32imc-unknown-none-elf
WORKDIR /project
COPY . .
RUN cargo fetch --locked
CMD ["bash", "scripts/reproduce.sh"]
