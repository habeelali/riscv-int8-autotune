#!/usr/bin/env python3
"""Refresh headline tables from executed benchmark artifacts, never fixed claims."""
import json,re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
a=json.loads((ROOT/'results/comparison.json').read_text());b=json.loads((ROOT/'results/competitor-comparison.json').read_text())
by={r['workload']:r for r in b['rows']}
labels=[('conv3x3','16×16×16 → 16×16×32, 3×3'),('pointwise','16×16×32 → 16×16×32, 1×1'),('depthwise','32×32×16 depthwise, 3×3'),('fully_connected','256 → 128 fully connected')]
text=f'''<!-- RESULTS_START -->
**Measured instruction-count improvement: {a['primary_geomean']:.3f}× over ESP-NN and
{b['primary_geomean_vs_stock_muriscv']:.3f}× over the fastest tested unmodified muRISCV-NN
scalar/portable configuration, geometrically averaged over the four requested shapes.**

Including a separately labeled local correction to muRISCV-NN's optional RV32
requantization assembly, the comparison is **{b['primary_geomean_vs_best_muriscv']:.3f}×**.
These are executed QEMU guest instruction ratios, not hardware cycles or measured
execution-time speedups. The complete eight-shape result versus ESP-NN is
{a['all_geomean']:.3f}×.

| Workload | ESP-NN | Best muRISCV-NN incl. local fix | Tuned Rust | Best mu / Rust |
|---|---:|---:|---:|---:|
'''
for n,label in labels:
 r=by[n];text+=f"| {label} | {r['esp_nn_instructions']:,} | {r['muriscv_best_instructions']:,} | {r['tuned_rust_instructions']:,} | {r['improvement_vs_best_muriscv']:.3f}× |\n"
text+='<!-- RESULTS_END -->'
p=ROOT/'README.md';s=p.read_text()
if '<!-- RESULTS_START -->' in s:s=re.sub(r'<!-- RESULTS_START -->.*?<!-- RESULTS_END -->',lambda _:text,s,flags=re.S)
else:s=re.sub(r'\*\*Measured result:.*?(?=!\[Measured instruction-count improvements\])',lambda _:text+'\n\n',s,flags=re.S)
s=re.sub(r'(?:A typical full search evaluates|The current full search evaluates) \d+ candidates',f"The current full search evaluates {a['candidates']} candidates",s)
s=re.sub(r'The selected main convolution uses \d+ bytes',f"The selected main convolution uses {next(r['scratch_bytes'] for r in a['workloads'] if r['workload']=='conv3x3')} bytes",s)
p.write_text(s)
