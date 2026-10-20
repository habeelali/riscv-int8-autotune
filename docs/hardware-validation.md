# Hardware validation: historical protocol

This page preserves the initial pre-hardware proposal. Physical ESP32-C3 results
are now available: [V5 audited evaluation](../results/hardware-v5/physical-audit/evaluation.md)
and [current reproduction procedure](reproducibility.md). The counter advice below
was an early plan, not the implemented C3 measurement backend. ESP32-C3 uses its
custom performance counter for SDK cycle timing; do not assume standard minstret
access or change its event mode without validating the SDK timing dependency.

## Original proposal (superseded)

1. Build a C3 ESP-IDF or esp-hal harness using the same immutable inputs, weight
   order, bias, multipliers, shifts, output activation limits and exported `run`.
2. Link original ESP-NN scalar C and generated Rust. Match release options and
   report compiler versions. Pin CPU clock, flash frequency and cache settings.
3. Allocate buffers with explicit placement/alignment. Run flash-resident and
   SRAM-resident code/data configurations separately. Include scratch setup.
4. Warm up, compare every output, and bracket only each kernel with correctly
   fenced `mcycle`/`minstret` reads. Measure/subtract the empty harness region.
   RV32 64-bit counters require high/low/high reads for long measurement periods.
5. Control interrupts or record their effects. Run at least 100 samples and
   report median, min, p95, distributions and device/firmware details. Check that
   counter subtraction cannot overflow unnoticed.
6. Measure stack with both compiler stack metadata where available and a runtime
   watermark; account for input/output activations, scratch, allocator overhead
   (if any), linked flash sections and ESP-IDF tasks separately.
7. Report hardware cycle ratios independently from the QEMU instruction ratios;
   do not transfer an instruction-count improvement into a latency/power claim.

Generated inference code requires scalar RV32IMC only. The optional measurement
harness requires CSR access; neither code nor QEMU MMIO startup is a drop-in
ESP32-C3 firmware image.
