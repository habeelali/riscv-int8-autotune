# Third-party attribution

* `models/person-detection`: TensorFlow Lite Micro pretrained person-detection
  model and two example BMP images, Apache-2.0. Exact upstream commit, URLs and
  hashes are in `models/person-detection/source.json`; the original license is
  retained. The importer separately records its metadata-only bias-axis repair.
  The common integer softmax is unmodified muRISCV-NN source (attributed below).

* `baselines/esp-nn`: Espressif Systems, Apache-2.0, upstream
  https://github.com/espressif/esp-nn commit
  `f1bfc45cb0edfdbe8fd4871f3abc0adfc7fe1c47`. Original notices and license
  retained. Sources are not modified. `baselines/c/include/string.h` is our
  freestanding header adapter, not a replacement implementation of the kernels.
* `qemu/qemu-plugin.h`: QEMU v10.0.0 public plugin header, original notices
  retained, GPL-2.0-or-later. `qemu/count-plugin.c` is also GPL-2.0-or-later.
  These optional host-side validation tools are separate from the MIT Rust
  workspace and exported kernels. See `LICENSES/GPL-2.0.txt`.
* The Rust requantization arithmetic follows the documented ESP-NN/TFLite
  integer contract. The implementation and source generator are project code;
  upstream C remains separately attributed above.
* Cargo dependencies and their licenses are recorded by `Cargo.lock` and each
  crate's published package. They are host-side dependencies only.

* `baselines/muriscv-nn`: Arm Limited and Chair of Electronic Design Automation,
  TUM; Apache-2.0, https://github.com/tum-ei-eda/muriscv-nn commit
  `f69494442858980be78180868d96c3524dbf26f9`. The checkout is unmodified.
  `results/upstream-overlays/` contains a separately labeled local correction
  to optional RV32 rounding assembly, retaining original notices and Apache-2.0
  terms. It is not presented as stock upstream source. The freestanding headers
  in `baselines/c/include/` only provide the declarations needed by the integer
  kernels; no used computation is mocked.

* `models/cifar10-resnet/model.tflite`: MLCommons Tiny pretrained quantized
  ResNet, from mlcommons/tiny commit
  `4addd0fa08d216e20637637874e084895f289da4`, Apache-2.0; original license
  retained as `LICENSE.mlcommons`. The model bytes are unmodified. The four
  JPEG fixtures are previews of CIFAR-10 test images served by uoft-cs/cifar10;
  source references, labels and SHA-256 hashes are in that directory's
  `source.json`. CIFAR-10 is by Alex Krizhevsky, Vinod Nair and Geoffrey Hinton,
  https://www.cs.toronto.edu/~kriz/cifar.html. Dataset imagery is separately
  attributed and is not relicensed as project MIT source. Preview pixels may
  differ from original CIFAR images; no official MLPerf accuracy claim is made.
