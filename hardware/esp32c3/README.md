# ESP32-C3 hardware diagnostic

This is real ESP-IDF bootable firmware, not the earlier QEMU-virt bare-metal
images. Target: ESP32-C3, any supported revision, at least 4 MB flash, 160 MHz,
DIO flash at 40 MHz. No sensor, camera, network or external wiring is required.
UART0 and secondary native USB Serial/JTAG both carry output at 115200 baud.

The merged binary contains bootloader at `0x0`, partition table at `0x8000` and
application at `0x10000`. Flash the **merged binary at offset `0x0`**. It replaces
the existing application. Do not use it on ESP32, ESP32-S2/S3, C2/C6 or other chips.

Nine backends share the same generic graph runner and scratch/activation arena:

1. ESP-NN, native depthwise first layer.
2. Original muRISCV-NN scalar.
3. Original muRISCV-NN portable algorithm, scalar RV32 instructions.
4. muRISCV-NN scalar with the separately attributed RV32 rounding correction.
5. muRISCV-NN portable algorithm with that correction.
6. Previous compact Rust model.
7. New exact-transform Rust candidate (bias folding, packed/chunk schedules,
   statically bounded rounding and per-layer scheduling as selected by QEMU).
8. rust_cache4: four-channel/two-pixel packed pointwise tiles, with channels
   outside pixels on layers 14, 16, 18, 20, 22, 24 and 26.
9. rust_cache2: two-channel/four-pixel tiles with the same channel-first order.

The additional variants reuse each 256–1,024-byte weight block across the image
instead of repeatedly scanning a 16–64 KB matrix for small pixel groups. This
is a cache hypothesis based on returned hardware profiles, not a confirmed
hardware speedup. Existing kernels and C baselines remain included. Tensor RAM
and maximum QEMU-measured stack are unchanged; the combined diagnostic image
is larger because it includes extra backends and differently packed parameters.
Do not use the combined image size to report standalone model flash usage.

Four embedded images include two upstream examples and their horizontal flips.
The entire expected intermediate tensor after each of 31 operators is stored
in flash and compared byte for byte. Correctness means matching LiteRT, not
dataset accuracy: the pretrained reference itself misses the person fixture.

The correctness/profiling pass emits **1116 `HW_LAYER` records**. The subsequent
benchmark executes seven rounds, rotating backend order, with an untimed repeat
immediately before each timed inference. It emits **252 `HW_RUN` records**.
The final `HW_DONE` record gives aggregate correctness, heap and worker stack
information. The workload task uses 16,384 bytes of stack; the reported minimum
free stack is a cumulative high-water mark for that shared task, not isolated
per-backend stack usage.

Whole-model cycle measurements include graph dispatch, padding, pooling,
reshape and softmax, and exclude printing, validation and RTOS delay calls.
The CPU clock is fixed; power management/radio startup are not enabled. Interrupts
remain enabled, so ticks/interrupts can add noise. Keep raw observations and
compare medians/minima rather than treating one sample as deterministic.
Layer profiling is diagnostic and contains serial/delay gaps between layers;
it does not reproduce the full-inference cache history. The diagnostic image
contains all nine backends and large test oracles, so its flash size is not the
size of a single-model production deployment.

Use [AGENT_PROMPT.md](AGENT_PROMPT.md) with the binary bundle. Capture all output,
not just averages. `capture.py` and `summarize.py` need only Python and PySerial.

## Rebuild

The measured kernel project and its model artifacts must already be present.
Install ESP-IDF **v5.4.2** with its pinned riscv32-esp-elf toolchain, export that
environment, and use Rust 1.90.0 with `riscv32imc-unknown-none-elf` installed.

```sh
python3 scripts/tune_hardware_cache.py
python3 hardware/esp32c3/prepare.py --cache-candidates
idf.py -C hardware/esp32c3 set-target esp32c3 build
```

The preparer writes only ignored generated files. It combines both Rust backends
into one no_std archive, namespaces the four C library variants, and preserves
the upstream implementations except the explicitly recorded rounding overlay.
Normal libc memory operations are shared by all backends in the hardware image.
No kernel heap allocation or vector/DSP instruction extension is introduced.

The binary was boot-tested in Espressif's ESP32-C3 QEMU machine, where all
functional checks passed. Emulator cycle values are not hardware results.
The previous candidate passed physical-board correctness but lost to muRISCV-NN
in cycles. Timing for these two new cache variants remains unmeasured.

Every boot emits HW_CAPTURE with an RTC boot counter and fresh RNG values. The
startup delay is eight seconds. capture.py refuses to overwrite existing files
and writes a host capture UUID, timestamp and log hash to .capture.json.
Preserve these identifiers to audit fresh capture independence.
