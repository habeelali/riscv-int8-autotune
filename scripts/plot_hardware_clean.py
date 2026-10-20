#!/usr/bin/env python3
"""A restrained, directly labelled figure from audited physical V5 measurements."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

ROOT=Path(__file__).resolve().parents[1]
data=json.loads((ROOT/'results/hardware-v5/physical-audit/comparison.json').read_text())
assert data['valid_captures']==3 and data['mismatches']==0
out=ROOT/'docs/figures/project';out.mkdir(parents=True,exist_ok=True)
plt.rcParams.update({'font.family':'DejaVu Sans','svg.fonttype':'none','pdf.fonttype':42,'text.color':'#20252b','axes.labelcolor':'#646b74','xtick.color':'#707780','font.size':11})
fig=plt.figure(figsize=(14,8),facecolor='white')
fig.text(.06,.90,'INT8 inference on ESP32-C3',fontsize=28,weight='medium')
fig.text(.06,.849,'Complete-model latency · scalar RV32IMC · 160 MHz CPU · 40 MHz DIO flash',fontsize=12,color='#656c76')
ax=fig.add_axes([.285,.265,.535,.455],facecolor='white')
ax.set_xlim(0,700);ax.set_ylim(-.55,1.65)
ax.set_xticks([0,100,200,300,400,500,600,700]);ax.set_yticks([])
for s in ax.spines.values():s.set_visible(False)
ax.tick_params(axis='x',length=0,pad=12,labelsize=10)
ax.grid(axis='x',color='#e9edf0',lw=.8);ax.set_axisbelow(True)
ax.set_xlabel('Inference time (ms)',labelpad=18,fontsize=11)
blue='#1558ad';gray='#7d8793'
for idx,(m,y) in enumerate(zip(data['models'],[1.1,0])):
    c=m['best_tested_c_ms'];rust=m['rust_ms']
    ax.plot([0,700],[y,y],color='#f1f3f5',lw=1,zorder=0)
    ax.plot([rust,c],[y,y],color='#c6cdd5',lw=3,zorder=1)
    ax.scatter([c],[y],s=130,color=gray,zorder=3,edgecolors='white',linewidth=2)
    ax.scatter([rust],[y],s=140,color=blue,zorder=3,edgecolors='white',linewidth=2)
    for val,color,label in [(rust,blue,'Generated Rust'),(c,gray,'Fastest tested C')]:
        ax.annotate(f'{val:.2f}',(val,y),xytext=(0,22),textcoords='offset points',ha='center',fontsize=15,fontfamily='DejaVu Sans Mono',color='#20252b')
        ax.annotate(label,(val,y),xytext=(0,-24),textcoords='offset points',ha='center',fontsize=9,color=color)
    yy=.265+((y+.55)/2.2)*.455
    fig.text(.06,yy+.008,['Person detection','CIFAR-10 ResNet'][idx],fontsize=16,weight='medium')
    fig.text(.06,yy-.028,['TFLM · 96 × 96 grayscale','MLCommons Tiny · 32 × 32 RGB'][idx],fontsize=10,color='#737b85')
    fig.text(.94,yy+.008,f"{m['c_over_rust']:.2f}×",ha='right',fontsize=26,color=blue)
    fig.text(.94,yy-.028,f"{100*(1-rust/c):.1f}% less time",ha='right',fontsize=10,color='#737b85')
fig.text(.06,.145,'3 reset sessions',fontsize=12,weight='medium')
fig.text(.30,.145,'480 timed inferences',fontsize=12,weight='medium')
fig.text(.59,.145,'2,256 tensor checks',fontsize=12,weight='medium')
fig.text(.84,.145,'0 mismatches',fontsize=12,weight='medium')
fig.add_artist(plt.Line2D([.06,.94],[.178,.178],transform=fig.transFigure,color='#dce1e6',lw=.8))
fig.text(.06,.09,'Four validation inputs per model; five observations per input/backend/boot. Values average per-input medians across three boots.',fontsize=9,color='#737b85')
fig.text(.06,.062,'C control: fastest tested ESP-NN / muRISCV-NN path per input. Speedups use geometric-mean input ratios. One board; hard resets, not power cycles.',fontsize=8.5,color='#737b85')
for suffix in ['png','svg','pdf']:
    fig.savefig(out/f'hardware-clean.{suffix}',dpi=180,facecolor='white')
    fig.savefig(out/f'hardware.{suffix}',dpi=180,facecolor='white')
plt.close(fig)
print(out/'hardware-clean.png')
