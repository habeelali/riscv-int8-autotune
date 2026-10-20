# Two-model ESP32-C3 diagnostic, v3

Flash the merged esp32c3-two-models-v3.bin at 0x0 on ESP32-C3 with at least 4 MB
flash. CPU 160 MHz, DIO flash 40 MHz, FreeRTOS interrupts enabled. This replaces
the application. USB Serial/JTAG (native C3 USB) and UART0 accept the same
115200-baud command protocol. The supplied agent prompt contains exact commands.

The firmware waits for the collector instead of automatically emitting all
benchmarks. This prevents losing early data and lets the host independently
choose each measurement. collect.py uses only PySerial and the Python standard
library; audit.py uses only the standard library. Neither requires ESP-IDF.

Two unmodified pretrained INT8 graphs are embedded: the prior TFLM 96×96 person
detector (with the already documented metadata-only bias-axis repair in its
host oracle), and MLPerf Tiny's 32×32 CIFAR-10 ResNet with residual connections.
Four fixed image fixtures per model are compared against LiteRT BUILTIN_REF
at every operator. The CIFAR fixtures are real test-image JPEG previews from the
dataset service and may differ from original CIFAR pixels; this is not an
accuracy evaluation or official MLPerf result.

Each graph runs ESP-NN optimized scalar C, stock scalar muRISCV-NN, the separately
labeled muRISCV-NN optional-rounding correction, and generated no_std Rust. Shared
C code performs graph dispatch, padding, pooling, reshape, softmax and residual
addition. Existing person kernels are unchanged. This is a mixed runtime with
Rust learned kernels. No heap allocation occurs inside kernels.

A complete capture has 752 intermediate checks, three requested-delay
calibrations, and 160 whole-model measurements: two models × four backends ×
four fixtures × five repetitions. There is one untimed warm inference before
each measurement. Host-triggered tick delays and sub-tick phase delays happen
outside the measured region. Ordering, delay values, tokens and timestamps are
recorded. Device reports cycles, start/end microseconds, observed tick phase,
output fingerprint, stack and heap. Correctness/printing/calibration/serial I/O
are outside the inference counter region.

The audit binds every request to its response and independent golden tensors,
checks raw/event/plan hashes, timer calibration and elapsed-clock agreement,
then checks separate boot identities and timing sequences. Three passing fresh
captures support repeatability on this board; they are not device attestation,
a dataset accuracy estimate or proof for other boards. Exact repeated timing
sequences are flagged. No confidence interval is invented from three boots.

Both models reuse the same 73,728-byte activation area and 38,416-byte padding
area (112,144 tensor bytes). ResNet's liveness allocator uses at most 49,152
activation bytes and retains residual tensors correctly. The worker stack is
16,384 bytes; its high-water mark is cumulative over all backends. Convolution
im2col uses explicit generated stack buffers (up to 2,304 bytes for selected
ResNet tiles), measured within that task. Driver/capture setup has additional
RTOS memory; static tensor use is unchanged. The diagnostic contains multiple
backends and large tensor oracles, so its image size is not standalone model
flash usage.

Rebuild from the full project: run scripts/resnet_model.py, then
scripts/benchmark_resnet.py; then scripts/benchmark_resnet.py --generic before
scripts/tune_resnet.py. Once milestones and selection exist, run
python3 hardware/esp32c3-v3/prepare.py and ESP-IDF v5.4.2
idf.py -C hardware/esp32c3-v3 build. Rust 1.90.0 target rv32imc is required.
The exact merged image and real collector protocol were executed on Espressif
ESP32-C3 QEMU. Emulator timing is not physical-board timing. New physical
repeatability and second-model timings require your returned captures.
