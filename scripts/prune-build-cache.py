#!/usr/bin/env python3
"""Remove rebuildable link intermediates; retain source, ELF, assembly, logs, maps and measurements."""
from pathlib import Path
root=Path(__file__).resolve().parents[1]/'results/build'
removed=0
for d in root.iterdir():
    if not (d/'result.json').exists():continue
    for p in d.iterdir():
        if p.suffix in ('.a','.o'):
            removed+=p.stat().st_size;p.unlink()
print(f'Removed {removed/1024**2:.1f} MiB of link intermediates; benchmark evidence retained.')
