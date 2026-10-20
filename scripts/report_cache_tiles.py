#!/usr/bin/env python3
"""Publish measured instruction/code trade-offs without inferring hardware latency."""
import csv,json,re
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
ROOT=Path(__file__).resolve().parents[1];D=ROOT/'results/model/cifar10-resnet';OUT=ROOT/'results/hardware-v4';OUT.mkdir(exist_ok=True)
names=['muriscv-fixed-asm','generated-0','tile8_b2p4','tile16_b2p4','tile16_b2p4_sram','tile16_b4p2','tile16_b2p4_block_sram'];rows=[]
for n in names:
 r=json.loads((D/n/'result.json').read_text());row={k:r[k] for k in ['mean_instructions_raw','text_bytes','rodata_bytes','static_ram_bytes','correct']};row['name']=n
 row['explicit_tile_scratch_bytes']=max([int(x) for x in re.findall(r'Scratch: (\d+) bytes',(D/n/'source.rs').read_text())],default=0) if (D/n/'source.rs').exists() else None
 rows.append(row)
with (OUT/'qemu-comparison.csv').open('w',newline='') as f:
 w=csv.DictWriter(f,list(rows[0]));w.writeheader();w.writerows(rows)
(OUT/'qemu-comparison.json').write_text(json.dumps(rows,indent=2)+'\n')
labels=['C: corrected mu','Previous Rust','Tile 8 / 2×4','Tile 16 / 2×4','Tile 16 + whole weights SRAM','Tile 16 / 4×2','Tile 16 + block SRAM (ablation)']
fig,ax=plt.subplots(1,2,figsize=(13,5),layout='constrained');colors=['#626c79','#4477aa','#44aa99','#228833','#44aa99','#44aa99','#aa6677']
for a,key,scale,title in [(ax[0],'mean_instructions_raw',1e6,'Executed guest instructions (millions)'),(ax[1],'text_bytes',1024,'Linked .text (KiB)')]:
 vals=[r[key]/scale for r in rows];a.barh(labels,vals,color=colors);a.invert_yaxis();a.set_xlabel(title);a.grid(axis='x',alpha=.2);a.set_axisbelow(True)
 for j,v in enumerate(vals):a.text(v+.15,j,f'{v:.2f}',va='center',fontsize=9)
 a.set_xlim(0,max(vals)*1.13)
fig.suptitle('Pretrained CIFAR-10 ResNet · RV32IMC QEMU\nInstruction/code trade-off; physical V4 latency is not measured yet')
fig.savefig(OUT/'cache-tile-tradeoffs.png',dpi=160);fig.savefig(OUT/'cache-tile-tradeoffs.svg');plt.close(fig)
