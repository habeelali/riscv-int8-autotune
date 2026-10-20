# Publish this project

The source package is a curated copy, not a Git history export. It contains all
authored code, preserved kernels/model parameters, pinned vendored baseline
sources, tests, measured JSON/CSV, original hardware captures including failures,
reports, figures and the social kit. Rebuildable caches, SDK/tool installations,
compiled firmware/ELF/static archives and private review prompts are excluded.
A file-hash manifest makes the snapshot inspectable. The original working tree
and failed experiments remain untouched.

## Create your repository

Extract `riscv-int8-autotune-source.zip`, enter its project directory and run:

```bash
git init -b main
git add .
git commit -m "Release measured RV32IMC INT8 kernel generation toolkit"
```

Create an empty repository on GitHub, then use the exact remote URL shown there:

```bash
git remote add origin YOUR_REPOSITORY_URL
git push -u origin main
```

The release source archive vendors baseline directories as ordinary files, so it
has no submodule initialization dependency. Original upstream pins and licenses
are retained. The development workspace may still use submodules.

Suggested repository name: `riscv-int8-autotune`.
Suggested description: "Measured scalar RV32IMC INT8 kernel generation in Rust, with QEMU autotuning and audited ESP32-C3 model benchmarks."
Suggested topics: `rust`, `risc-v`, `esp32-c3`, `tinyml`, `int8`, `autotuning`,
`embedded`, `no-std`, `qemu`, `neural-network`.

Set your preferred author/maintainer identity in `CITATION.cff`; add the actual
repository URL and release URL after publishing. No person, affiliation, DOI or
repository ownership was invented in the package.

## What to put in a GitHub Release

- The source archive and its SHA-256 sidecar.
- An optional separately attached exact V5 benchmark firmware bundle, not embedded
  in normal Git source history. Preserve its original image hash and caveats.
- The LinkedIn carousel and cover if you want convenient shareable downloads.

The V5 diagnostic image is not a compact production application; it contains
multiple backends, golden tensors and fixtures. Passing physical sessions do not
resolve the intermittent simulation stalls. Retain those facts in release notes.

The fast CI workflow checks source and audits saved physical evidence. The full
Docker/QEMU research workflow is manual because it is expensive and its Docker
recipe has not been verified here. Neither CI workflow fabricates board runs.

## Rebuild a clean source package

```bash
python3 scripts/package_repository.py --out dist/riscv-int8-autotune-source.zip
```

The command copies selected source/evidence without deleting or pruning your
working files. It checks for oversized files and local private-key formats, then
writes a manifest and archive checksum. Review your own identity/device metadata
before making a fork public; raw benchmark captures intentionally retain their
original contents.
