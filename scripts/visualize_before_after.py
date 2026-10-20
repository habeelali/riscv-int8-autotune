#!/usr/bin/env python3
"""Render a normalized dot-matrix comparison from executed benchmark results."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
ROOT=Path(__file__).resolve().parents[1]
data=json.loads((ROOT/'results/comparison.json').read_text())
by_name={r['workload']:r for r in data['workloads']}
items=[('conv3x3','3×3 convolution','16×16×16 → 16×16×32'),('pointwise','1×1 convolution','16×16×32 → 16×16×32'),('depthwise','Depthwise 3×3','32×32×16'),('fully_connected','Fully connected','256 → 128')]
bg='#101923';panel='#17232f';ink='#f1f5fa';muted='#a9bacb';before='#79a5d1';after='#3fe0b2';outline='#344858'
plt.rcParams.update({'font.family':'DejaVu Sans','text.color':ink})
fig=plt.figure(figsize=(14,11.8),facecolor=bg)
fig.text(.045,.952,'BEFORE → AFTER',fontsize=30,weight='bold')
fig.text(.045,.914,'Same numerical outputs. Fewer instructions.',fontsize=17,color=muted)
fig.text(.94,.954,f"{data['primary_geomean']:.2f}×",fontsize=35,color=after,weight='bold',ha='right')
fig.text(.94,.921,'geometric mean improvement',fontsize=11,color=muted,ha='right')
fig.text(.295,.862,'BEFORE · ESP-NN C',fontsize=12,color=before,weight='bold')
fig.text(.575,.862,'AFTER · TUNED RUST',fontsize=12,color=after,weight='bold')
fig.text(.815,.862,'INSTRUCTIONS SAVED',fontsize=11,color=muted,weight='bold')
xs=[i%10 for i in range(100)];ys=[9-i//10 for i in range(100)]
for j,(name,label,shape) in enumerate(items):
    r=by_name[name];bottom=.654-j*.19
    fig.add_artist(FancyBboxPatch((.03,bottom-.003),.94,.174,boxstyle='round,pad=0.008,rounding_size=0.013',transform=fig.transFigure,facecolor=panel,edgecolor='none',zorder=0))
    fig.text(.052,bottom+.106,label,fontsize=17,weight='bold')
    fig.text(.052,bottom+.076,shape,fontsize=11,color=muted)
    frac=r['tuned_instructions']/r['c_instructions'];remaining=round(frac*100)
    for xpos,n,color in [(.295,100,before),(.575,remaining,after)]:
        ax=fig.add_axes([xpos,bottom+.033,.17,.127],facecolor=panel,zorder=2)
        ax.scatter(xs,ys,s=30,facecolors=panel,edgecolors=outline,linewidths=.85)
        ax.scatter(xs[:n],ys[:n],s=30,facecolors=color,edgecolors='none')
        ax.set_xlim(-.8,9.8);ax.set_ylim(-.8,9.8);ax.set_aspect('equal');ax.axis('off')
    fig.text(.295,bottom+.009,f"{r['c_instructions']:,}",fontsize=12,color=before,weight='bold')
    fig.text(.575,bottom+.009,f"{r['tuned_instructions']:,}",fontsize=12,color=after,weight='bold')
    fig.text(.815,bottom+.09,f'{(1-frac)*100:.1f}%',fontsize=29,color=after,weight='bold')
    fig.text(.815,bottom+.055,f"{r['instruction_improvement']:.2f}× improvement",fontsize=11,color=ink)
    fig.text(.815,bottom+.025,f"−{r['c_instructions']-r['tuned_instructions']:,} instructions",fontsize=10,color=muted)
fig.text(.045,.049,'Each matrix = 100% of that operator’s C baseline. Filled dots = instructions still used.',fontsize=11,color=muted)
fig.text(.045,.028,'Dots are rounded to 1%; numeric labels are exact. QEMU guest instructions ≠ hardware cycles or elapsed time.',fontsize=10,color=muted)
out=ROOT/'docs/figures';out.mkdir(exist_ok=True)
for ext in ('png','svg','pdf'):fig.savefig(out/f'before-after.{ext}',dpi=170,facecolor=bg)
plt.close(fig)
print(out/'before-after.png')
