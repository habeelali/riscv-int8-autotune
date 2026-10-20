# V4: hardware-directed generalization, awaiting physical timing

The V3 person-detector win (1.451× against fastest tested C) and ResNet loss
(0.910×) remain the latest physical results. This iteration implements a reusable
cache-tiled convolution family and an empirical hardware selector. **It does not
yet establish a second-model hardware win.** The previous person export and its
parameters were preserved byte-for-byte.

## What changed

The previous ResNet generator used two-position INT16 partial im2col, four output
channels per register block, and ordinary OHWI weights. The new family uses an
8- or 16-position signed-INT8 patch tile, offline channel-blocked weights, and
2-channel × 4-position or 4-channel × 2-position microtiles. Each packed weight
block is reused across the larger tile. All arithmetic remains scalar RV32IMC.
No heap allocation, SIMD, hand-written assembly, approximate quantization, pruning
or changed trained weights are involved.

Padding is materialized with the original signed INT8 zero point. Consequently,
`folded_bias = bias + input_offset * sum(weights)` is exact for border positions as
well as interior positions. The importer proves the absolute accumulator bound
and quantizer range before removing runtime parameter scans. Twenty-four actual
generated host programs were compared with the independent integer reference,
covering extreme zero points, odd channel/reduction lengths, spatial tails,
strides 1/2/3, padding 0/1/2, and fully connected layers. Every candidate also ran
all 64 intermediate checks (four inputs × 16 operators) twice in RV32IMC QEMU.

A separate graph-level candidate copies an entire layer's packed weights into the
unused tail of the existing 38,416-byte padding workspace before its kernel. The
copy is INSIDE the timed graph and layer interval. An offline proof checks that
live padded input and staged weights cannot overlap. The largest weight tensor is
36,864 bytes; its input lives in the activation arena, leaving the padding buffer
available. For layers whose input already occupies the padding buffer, weights
fit above the live input. This adds no static tensor memory. It is a standard
memory-placement optimization applied using graph liveness, not a claimed novel
Rust algorithm. The reusable Rust kernel accepts any appropriately packed weight
slice; graph orchestration performs staging.

## Executed measurements (QEMU guest instructions, not hardware cycles)

| ResNet implementation | Mean guest instructions | Linked text bytes | Explicit tile scratch bytes |
|---|---:|---:|---:|
| Corrected muRISCV-NN C | 54,045,386.75 | 28,706 | backend-specific |
| Previous Rust | 45,374,096.50 | 41,530 | 2,304 |
| Tile 8, block 2 × pixels 4 | 45,159,787.00 | 41,832 | 4,608 |
| Tile 16, block 2 × pixels 4 | 44,996,311.00 | 38,054 | 9,216 |
| Tile 16 + whole-weight SRAM staging | 45,049,836.00 | 38,498 | 9,216 |
| Tile 16, block 4 × pixels 2 | 45,115,604.25 | 38,226 | 9,216 |
| Tile 16 + per-block SRAM staging, retained ablation | 45,685,772.75 | 38,220 | 10,368 |

The lowest instruction count is 0.83% below previous Rust and 1.201× fewer
instructions than corrected muRISCV-NN. Its linked text is 8.37% below previous
Rust, but still larger than C. These are not latency claims. The per-block staging
ablation regresses instructions and is retained. Whole-weight staging adds copy
instructions; its possible benefit is reduced flash stalls, which QEMU does not
model sufficiently for selection.

All these virt benchmark ELFs have 112,160 bytes of static RAM. The 112,144-byte
tensor arena is unchanged; dynamic stack usage is additional. The diagnostic task
reserves 16,384 bytes, as in V3. The final emulator run observed at least 5,772 bytes
free stack and 192,572 bytes free heap. These are emulator observations; physical
watermarks are captured separately. Additional profiling/selection arrays consume
RAM. Diagnostic flash includes every candidate, multiple packed parameter copies,
and golden tensors and must not be presented as production model size.

[Measured instruction/code plot](cache-tile-tradeoffs.png),
[CSV](qemu-comparison.csv), [workspace proof](workspace-liveness-proof.json).

## Assembly findings

The disassembly of ResNet layer 9 verifies an eight-multiply inner loop:

- Previous Rust: four signed byte weight loads at widely separated OHWI addresses
  and two signed halfword patch loads.
- Block-2/pixels-4 tile: two adjacent signed byte weight loads and four signed byte
  patch loads, eight scalar `mul`, register accumulators, pointer increments and
  one loop branch. No accumulator spills occur in this inner reduction loop.
- Whole-weight staging uses the same kernel body with a weight pointer into SRAM;
  the timed copy is in `resnet_execute`, not in the reduction loop.

This halves the weight loads per multiply for this microtile while increasing
SRAM patch loads, and makes adjacent weight accesses contiguous. Reuse across
microtiles is expected to help cache residency. Cache misses and stall cycles were
NOT measured, so these are mechanistic explanations to test, not hardware proof.
Assembly for the final ELF and its SHA-256 are saved under `assembly/`.

## Selection and validation protocol

V4 includes ESP-NN, stock scalar muRISCV-NN, the explicitly attributed locally
corrected muRISCV-NN variant, incumbent Rust, and four Rust candidates. The C
baselines keep their stock weight-memory placement; whole-weight SRAM staging is
an additional Rust-graph optimization. Any gain cannot be attributed to the Rust
language alone. MicroFlow, CubeCL and other unexecuted projects are not included
in performance claims.

Training uses ONLY inputs 0 and 1, three repeats per layer/backend/input. Layer
profiles include padding and kernel work. The deterministic selector requires an
alternative to improve both training-input medians by at least 2%. This margin is
not a statistical confidence bound. It creates a Rust-only map and a separately
labeled C/Rust hybrid map. Shared non-learned graph operators remain common C.
Person Rust layers retain their measured incumbent implementation.

Both composed graphs then undergo full intermediate correctness checking on all
four inputs, followed by five complete-inference repetitions per input/backend.
Inputs 2 and 3 are held out from selection. Whole-model timing excludes profiling,
printing, validation and randomized waits. Later reset captures reuse the FIRST
selection without retuning. The audit verifies the exact selection against its
included training capture and reports held-out ratios against both fastest tested
C and incumbent Rust. It does not promote a layer mixture merely because its
individual layers won, or hide a whole-model regression.

The final merged binary passed two actual ESP32-C3 QEMU sessions, including the
frozen-selection path: 3,760 tensor comparisons, 800 timed graph invocations,
192 profiling graph invocations and six clock calibrations, all zero mismatches.
Emulator timing is explicitly excluded from hardware evidence. Five audit tests
(control plus four corrupted/missing-evidence cases) passed. Workspace Rust tests,
formatting, clippy with warnings denied, and eight existing Python tests passed.

The emulator-selected Rust mixture was also regenerated, exported, linked and
run independently: 45,302,367 mean guest instructions and 42,528 text bytes. This
is larger than both previous Rust and the best fixed tile. It is a functional
export test, NOT a recommended hardware schedule. All results are preserved.

## What remains to establish generalization

Flash the bundled final binary and return three physical reset captures following
`hardware/esp32c3-v4/AGENT_PROMPT.md`. Require correct outputs, frozen choices,
independent clock/phase evidence, held-out complete-model gains, and explicit
memory/code trade-offs. The new image's incumbent measures control for code-layout
and instrumentation differences from V3. Compare candidates within this image.

After hardware selection, `scripts/export_hardware_v4.py` regenerates the selected
Rust model and remeasures code size, RAM and guest instructions. Production code
layout can alter cache behavior, so remeasure the compact deployment on hardware
before promoting it. The incumbent export remains available. Selection does not
yet solve a global flash-budget-constrained layer-mixture problem.

Two tiny fixture sets on one board cannot establish universal model or board
performance, task accuracy, or research novelty. The concrete contribution of
this iteration is a correct reusable cache-tiling family plus a hardware-feedback
selection/validation workflow that can expose rather than suppress the previous
failure to generalize.

## Physical return update

The uploaded V4 physical run failed to complete. Three valid-build sessions each
passed 1,504 fixed tensor checks and then stalled on ResNet whole-weight SRAM
staging (backend 6) during training. There are no frozen selections or held-out
RUN observations. The new tile training profiles are promising, but no physical
V4 speedup can be claimed. See [independent partial audit](physical-audit/evaluation.md)
and the preserved raw evidence. Recovery V4.1 quarantines this candidate; the
underlying physical cause remains undetermined.
