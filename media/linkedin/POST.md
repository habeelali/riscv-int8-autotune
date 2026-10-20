I built a Rust tool that generates and tunes INT8 kernels for scalar RISC-V—and tested the results on an actual ESP32-C3.

The goal was to measure what specialized code can do on a tiny processor without vector instructions or packed SIMD.

Against the fastest tested ESP-NN/muRISCV-NN C controls, complete-model inference took:

• Person detection: 376.31 → 259.23 ms (1.45×)
• CIFAR-10 ResNet: 620.38 → 354.69 ms (1.75×)

That is a 1.59× geometric mean across the two models, verified over three hard-reset sessions on one board. All 2,256 intermediate tensor checks and 480 timed inferences passed with zero mismatches.

The useful part is the tool: give it a workload TOML, and it checks the environment, measures C and generic Rust, generates different kernels, validates them in QEMU, ranks them under flash/SRAM budgets, and exports a standalone no_std crate with an HTML report and machine-readable evidence.

The techniques are established: blocking, data reuse, tiling, packing and exact quantization specialization. This is not a claim that Rust automatically beats C. Generic Rust was slower; some tuned candidates also lost. Those results are kept.

There is more to do: the hardware evidence covers one board and known validation fixtures. Intermittent ESP32-C3 simulation stalls remain undiagnosed; I am not treating successful hardware runs as an explanation for them.

Source, raw measurements, plots and reproduction instructions: add your published repository URL here before posting.

#Rust #RISCV #EmbeddedSystems #TinyML #OpenSource

---

Upload `carousel.pdf` as a LinkedIn document, or use `linkedin-cover.png` as a
single image. Each carousel PNG is 1440 × 1440; the cover is 1440 × 756. The SVGs
are editable. Replace the repository line with your actual URL. Use the PDF or
individual slides, rather than screenshotting graphs from the README.
