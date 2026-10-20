# Reproduce the project

There are three different tasks: use the tool for a new workload, rerun research
experiments in generic RV32IMC QEMU, and collect ESP32-C3 physical timings. They
use different environments and metrics. Reading stored measurements is not a new
benchmark run.

## New workloads

Follow [the tool guide](tool.md). After setup, the shortest path is:

```bash
cargo install --locked --path crates/autotuner
riscv-int8 doctor
riscv-int8 init demo.toml --operator pointwise
riscv-int8 build demo.toml --out artifacts/demo --max-candidates 24
```

The CLI runs genuine compiler/QEMU jobs, preserves each candidate and validates a
standalone export. It does not need a physical board or Python plotting packages.

## Host and microbenchmark setup

Validated host: Linux x86-64; Rust 1.90.0; target
`riscv32imc-unknown-none-elf`; GCC 14.2.0; binutils 2.44; generic QEMU 10.0.13.
Python 3.11+ and a host C compiler are required. Dependencies are pinned in
`Cargo.lock`, `rust-toolchain.toml`, the setup script and Python requirements.

```bash
bash scripts/setup.sh
source scripts/env.sh
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
python3 -m unittest discover -s tests -v
bash scripts/reproduce.sh
```

The full research script measures baseline and generic Rust before tuning, checks
upstream configurations, calibrates the instruction counter and validates the
optional QEMU execution plugin. Results include every candidate. See the
[experiment guide](experiment-guide.md) for all objectives and quantization details.
The Dockerfile and optional full-evaluation CI recipe are supplied, but no claim
is made that Docker was executed and verified in this session.

Generic QEMU uses `virt`/`lowrisc-ibex`, scalar RV32IMC inference and CSR
instrumentation, with `-icount shift=0,align=off,sleep=off`. The measured region is
the kernel callback after empty-call overhead subtraction. This counts dynamically
executed guest instructions; it is not a hardware cycle model. Known-length loops
calibrate the counter and an independent plugin cross-check is retained.

## Pretrained-model research workflows

```bash
python3 -m pip install -r requirements-model.txt
bash scripts/reproduce-model.sh
bash scripts/reproduce-flash.sh
python3 scripts/benchmark_resnet.py
python3 scripts/benchmark_resnet.py --generic
python3 scripts/tune_resnet.py
python3 scripts/tune_cache_tiles.py
python3 scripts/test_cache_tiles.py
```

These research workflows retain separate manifests, packed parameters, reference
vectors and fixtures. They are not a general arbitrary-TFLite importer. The person
model requires its documented metadata-only repair for the independent LiteRT
oracle; model weights are unchanged. The four images per model establish limited
functional equivalence, not classification accuracy across a dataset.

## Recheck existing physical evidence

```bash
bash scripts/audit-v5.sh
python3 scripts/test_hardware_v5_audit.py
python3 scripts/report_hardware_v5.py
python3 -m pip install -r requirements.txt
python3 scripts/presentation.py
```

`report_hardware_v5.py --image path/to/esp32c3-final-benchmark-v5.bin` additionally
checks the merged image against its recorded SHA-256. The report can be rebuilt
without the binary; source/oracle identity and returned archive hashes are always
checked. The media script hashes its data inputs in
`docs/figures/project/data-provenance.json`.

The hardware auditor checks raw/event equality and hashes, challenge and command
order, all reference tensor fingerprints, response sequencing, requested/random
phases, delay calibration, cycle/wall consistency and host/device elapsed time.
Distinct nonces alone do not establish independent timing. See
[physical evaluation](../results/hardware-v5/physical-audit/evaluation.md).

## Build and collect new ESP32-C3 evidence

The hardware image uses ESP-IDF **v5.4.2**, commit
`f5c3654a1c2d2a01f7f67def7a0dc48e691f63c0`, Espressif GCC
`esp-14.2.0_20241119`, stable Rust 1.90.0 and esptool 4.12.0.
Use the normal ESP-IDF installation/export procedure for your host first.
The already included generated headers preserve the measured parameters; prepare
regenerates them and the Rust library from the retained sources.

```bash
source /path/to/esp-idf-v5.4.2/export.sh
source scripts/env.sh
python3 hardware/esp32c3-v5/prepare.py
idf.py -C hardware/esp32c3-v5 build
```

To merge a newly built image with standard ESP-IDF offsets:

```bash
python3 -m esptool --chip esp32c3 merge_bin \
 --flash_mode dio --flash_freq 40m --flash_size 4MB --fill-flash-size 4MB \
 -o esp32c3-final-benchmark-v5.bin \
 0x0 hardware/esp32c3-v5/build/bootloader/bootloader.bin \
 0x8000 hardware/esp32c3-v5/build/partition_table/partition-table.bin \
 0x10000 hardware/esp32c3-v5/build/riscv_int8_final_benchmark.bin
```

Do not assume that a rebuild has the exact archived binary hash, or reuse old
physical results as its timing. Build/source/provenance changes require new evidence.
The historical packaging script has an exact-emulator-image acceptance guard;
it is not a generic release command for arbitrary new builds.

Follow [the hardware procedure](../hardware/esp32c3-v5/AGENT_PROMPT.md) for flashing,
resetting and collecting. The source collector automatically reads its freshly generated oracle from
`main/generated/oracle.json` when no adjacent bundle oracle exists. For example:

```bash
python3 hardware/esp32c3-v5/collect.py --port COM6 \
 --output captures/new-boot-1
```

Use fresh prefixes and challenges; preserve all four sidecars and failures. Never
pass `--emulator` for physical logs. Three hard resets do not substitute for power
cycles or independent boards.

Espressif QEMU 9.0.0 (`esp_develop_9.0.0_20240606`) is separate from the generic
QEMU backend. Historical board-simulation scripts retain environment-specific
paths and are diagnostic records, not the supported `build` backend. Some complete
V5 board-simulation sessions pass and others stall. The cause remains unknown;
no simulation regression pass or emulator-specific diagnosis is claimed.
