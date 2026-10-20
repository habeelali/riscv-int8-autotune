#!/usr/bin/env python3
"""Render repository and social graphics from saved, audited measurements only."""
import csv
import hashlib
import json
import math
from pathlib import Path
import statistics as st

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
from matplotlib.patches import FancyBboxPatch
import numpy as np

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT/'docs/figures/project'
SOCIAL = ROOT/'media/linkedin'
OUT.mkdir(parents=True, exist_ok=True)
SOCIAL.mkdir(parents=True, exist_ok=True)
inputs = ['results/hardware-v5/physical-audit/comparison.json',
          'results/hardware-v5/physical-audit/summary.json',
          'results/hardware-v5/physical-audit/runs.csv',
          'results/comparison.json', 'results/flash/summary.json',
          'results/hardware-v4/qemu-comparison.json']
def load(p): return json.loads((ROOT/p).read_text())
hw = load(inputs[0]); audit = load(inputs[1]); micro = load(inputs[3]); flash = load(inputs[4]); tiles = load(inputs[5])
assert audit['independent_repeatability_evidence'] and audit['valid_captures'] == 3
assert not audit['emulation_only'] and hw['mismatches'] == 0
models = hw['models']
NAVY='#101c31'; TEAL='#39e3b6'; BLUE='#80a9ff'; WHITE='#f1f5fb'; GRAY='#a4b3c9'; CORAL='#ffb29a'; GRID='#28384f'
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':11,'axes.facecolor':NAVY,'figure.facecolor':NAVY,'savefig.facecolor':NAVY,'text.color':WHITE,'axes.labelcolor':GRAY,'xtick.color':GRAY,'ytick.color':WHITE,'axes.edgecolor':GRID,'axes.spines.top':False,'axes.spines.right':False,'axes.spines.left':False,'axes.spines.bottom':False,'svg.fonttype':'none','svg.hashsalt':'riscv-int8-autotune','pdf.fonttype':42})

def save(fig, name):
    for suffix in ['svg','png','pdf']:
        fig.savefig(OUT/f'{name}.{suffix}', dpi=200, bbox_inches='tight', metadata={'Creator':'riscv-int8-autotune'})
    plt.close(fig)

def heading(fig, eyebrow, title, subtitle):
    fig.text(.06,.94,eyebrow.upper(),color=TEAL,fontsize=10,weight='bold')
    fig.text(.06,.875,title,fontsize=25,weight='bold')
    fig.text(.06,.825,subtitle,color=GRAY,fontsize=10)

def foot(fig, text): fig.text(.06,.035,text,fontsize=9,color=GRAY)

fig=plt.figure(figsize=(12,6.8))
heading(fig,'01 / Physical hardware','Inference time','ESP32-C3 · 160 MHz CPU · DIO flash 40 MHz · fixed kernels · three hard-reset sessions')
for m,(model,ax) in enumerate(zip(models,[fig.add_axes([.22,.51,.66,.18]),fig.add_axes([.22,.19,.66,.18])])):
    vals=[model['best_tested_c_ms'],model['rust_ms']]
    ax.barh([1,0],vals,color=[BLUE,TEAL],height=.45)
    ax.set_yticks([1,0],['Fastest tested C','Generated Rust']);ax.set_xlim(0,740);ax.set_xticks([0,200,400,600]);ax.grid(axis='x',color=GRID);ax.set_axisbelow(True)
    ax.set_title(model['model'],loc='left',pad=15,weight='bold',fontsize=13)
    for y,v in zip([1,0],vals): ax.text(v+10,y,f'{v:.2f} ms',va='center',fontsize=12,weight='bold')
    ax.text(1,1.16,f"{model['c_over_rust']:.3f}×",transform=ax.transAxes,ha='right',fontsize=21,weight='bold',color=TEAL)
    ax.set_xlabel('Complete inference time (ms) · lower is better',fontsize=10)
foot(fig,'Per-input medians of five runs; four fixtures per model; three boots. C controls: ESP-NN and two muRISCV-NN paths.')
save(fig,'hardware')

fig=plt.figure(figsize=(12,7.5));heading(fig,'02 / Repeatability','Inference timing','Individual Rust inferences across all three physical sessions · no discarded observations')
with (ROOT/inputs[2]).open() as f: raw=list(csv.DictReader(f))
for idx,ax in enumerate([fig.add_axes([.13,.49,.82,.23]),fig.add_axes([.13,.17,.82,.23])]):
    for boot,color,offset in [(1,BLUE,-.18),(2,TEAL,0),(3,CORAL,.18)]:
        r=[x for x in raw if int(x['model'])==idx and int(x['backend'])==3 and Path(x['capture']).name==f'boot-{boot}']
        for sample in range(4):
            values=[float(x['inference_ms']) for x in r if int(x['sample'])==sample]
            ax.scatter(sample+offset+np.linspace(-.035,.035,len(values)),values,s=20,alpha=.8,color=color,label=f'Boot {boot}' if sample==0 else None)
    ax.set_title(models[idx]['model'],loc='left',fontsize=13,weight='bold');ax.set_xticks(range(4),[f'Fixture {i}' for i in range(4)])
    ax.set_ylabel('Time (ms)');ax.grid(axis='y',color=GRID);ax.set_axisbelow(True);ax.ticklabel_format(useOffset=False,axis='y')
    if idx==0:ax.legend(ncol=3,frameon=False,fontsize=9,labelcolor=GRAY,loc='upper right')
foot(fig,'Vertical axes are zoomed to expose variation. Five observations per fixture/boot. One board; resets are not power cycles.')
save(fig,'repeatability')

fig=plt.figure(figsize=(12,6.8));heading(fig,'03 / Instruction count','Instruction count','Primary scalar RV32IMC microbenchmarks · executed QEMU guest instructions, not hardware cycles')
ax=fig.add_axes([.18,.22,.76,.49]);names=['3×3 conv','1×1 conv','Depthwise','Fully connected'];rows=micro['workloads'][:4]
# Select by workload name rather than relying on an incidental file order.
rows=[next(x for x in micro['workloads'] if x['workload']==n) for n in ['conv3x3','pointwise','depthwise','fully_connected']]
x=np.arange(4)
for off,key,label,color in [(-.24,'c_instructions','ESP-NN C',BLUE),(0,'generic_instructions','Generic Rust',CORAL),(.24,'tuned_instructions','Generated Rust',TEAL)]:
    vals=[r[key] for r in rows];ax.bar(x+off,vals,width=.22,color=color,label=label)
ax.set_yscale('log');ax.set_xticks(x,names);ax.set_ylabel('Executed instructions · logarithmic scale');ax.grid(axis='y',color=GRID);ax.set_axisbelow(True);ax.legend(frameon=False,ncol=3,labelcolor=GRAY,loc='upper right')
foot(fig,'Generic Rust loses on every primary shape. Tuned Rust / C comparisons are implementation-specific, not language benchmarks.')
save(fig,'instructions')

fig=plt.figure(figsize=(12,6.8));heading(fig,'04 / Flash study','Flash usage','Archived person-model deduplication study · separate from the V5 hardware configuration')
for ax,key1,key2,title in [(fig.add_axes([.1,.25,.36,.44]),'code_bytes_before','code_bytes_after','Generated kernel code'),(fig.add_axes([.57,.25,.36,.44]),'deploy_image_bytes_before','deploy_image_bytes_after','Deployment image')]:
    vals=[flash[key1]/1024,flash[key2]/1024];bars=ax.bar(['Before','After'],vals,width=.5,color=[BLUE,TEAL]);ax.set_ylim(0,max(vals)*1.3);ax.set_title(title,fontsize=14,loc='left',pad=15);ax.set_ylabel('KiB');ax.grid(axis='y',color=GRID);ax.set_axisbelow(True)
    for bar,v in zip(bars,vals):ax.text(bar.get_x()+bar.get_width()/2,v+max(vals)*.035,f'{v:.2f}',ha='center',weight='bold')
    ax.text(.98,.92,f'−{100*(1-vals[1]/vals[0]):.1f}%',transform=ax.transAxes,ha='right',fontsize=23,color=TEAL,weight='bold')
foot(fig,'All four whole-model and 124 layer/input instruction deltas are zero. This is not an isolated V5 production-flash measurement.')
save(fig,'flash')

fig=plt.figure(figsize=(12,6.8));heading(fig,'05 / Trade-offs','Instructions and code size','CIFAR-10 ResNet · QEMU instruction count versus linked text · actual measured candidates')
ax=fig.add_axes([.12,.22,.8,.48]);labels={'muriscv-fixed-asm':'Corrected C','generated-0':'Previous Rust','tile8_b2p4':'Tile 8','tile16_b2p4':'Tile 16 / 2×4','tile16_b2p4_sram':'Whole staging','tile16_b4p2':'Tile 16 / 4×2','tile16_b2p4_block_sram':'Block staging'}
for i,r in enumerate(tiles):
    color=TEAL if r['name']=='tile16_b2p4' else (BLUE if i==0 else (CORAL if 'sram' in r['name'] else GRAY))
    ax.scatter(r['text_bytes']/1024,r['mean_instructions_raw']/1e6,s=100,color=color,zorder=3)
    label_positions={'tile16_b2p4':(34.4,44.65),'tile16_b2p4_sram':(38.6,46.8),'tile16_b4p2':(33.3,45.9),'tile16_b2p4_block_sram':(35.7,48.1),'generated-0':(39.6,48.1),'tile8_b2p4':(41,46.0),'muriscv-fixed-asm':(29,54.5)}
    ax.annotate(labels[r['name']],(r['text_bytes']/1024,r['mean_instructions_raw']/1e6),xytext=label_positions[r['name']],textcoords='data',fontsize=9,color=color,arrowprops=dict(arrowstyle='-',color=color,lw=.8))
ax.set_xlim(26,44);ax.set_ylim(44,56);ax.set_xlabel('Linked text (KiB) · lower is smaller');ax.set_ylabel('Executed guest instructions (millions) · lower is better');ax.grid(color=GRID);ax.set_axisbelow(True)
foot(fig,'Staging variants remain failed/experimental hardware paths. Their simulated measurements do not establish physical reliability.')
save(fig,'tradeoffs')

# Social slides use fixed canvas sizes: no crop and readable type on mobile.
def canvas(w=12,h=12):
    fig=plt.figure(figsize=(w,h),dpi=120)
    fig.text(.075,.925,'RISC-V  /  RUST  /  EMBEDDED ML',color=TEAL,fontsize=13,weight='bold')
    fig.text(.075,.045,'riscv-int8-autotune',color=WHITE,fontsize=13,weight='bold')
    return fig

def social_save(fig,name,pdf=None):
    fig.savefig(SOCIAL/f'{name}.png',dpi=120)
    fig.savefig(SOCIAL/f'{name}.svg')
    if pdf:pdf.savefig(fig)
    plt.close(fig)

with PdfPages(SOCIAL/'carousel.pdf') as pdf:
    fig=canvas();fig.text(.075,.77,'Scalar RISC-V.\nFaster INT8 inference.',fontsize=44,weight='bold',linespacing=1.3)
    fig.text(.075,.48,f"{hw['equal_model_geomean']:.2f}×",color=TEAL,fontsize=106,weight='bold')
    fig.text(.085,.405,'geometric-mean speedup across two models',fontsize=20,color=WHITE)
    fig.text(.075,.255,'Generated Rust kernels vs fastest tested C\nESP32-C3 · physical board measurements',fontsize=21,color=GRAY,linespacing=1.65)
    fig.text(.075,.12,'01 / 04    Three hard-reset sessions · one board · fixed kernels',fontsize=12,color=GRAY)
    social_save(fig,'01-cover',pdf)
    fig=canvas();fig.text(.075,.79,'Two real models.\nComplete inference.',fontsize=42,weight='bold',linespacing=1.3)
    for model,y in zip(models,[.57,.32]):
        fig.text(.075,y,model['model'],fontsize=21,color=GRAY)
        fig.text(.075,y-.075,f"{model['best_tested_c_ms']:.1f} → {model['rust_ms']:.1f} ms",fontsize=33,weight='bold')
        fig.text(.76,y-.075,f"{model['c_over_rust']:.2f}×",fontsize=29,color=TEAL,weight='bold')
    fig.text(.075,.12,'02 / 04    Fastest of ESP-NN and two tested muRISCV-NN paths',fontsize=12,color=GRAY)
    social_save(fig,'02-models',pdf)
    fig=canvas();fig.text(.075,.79,'Speed means little\nwithout correct outputs.',fontsize=39,weight='bold',linespacing=1.3)
    for value,label,y in [('2,256','intermediate tensor checks',.56),('480','timed whole-model inferences',.37),('0','mismatches in three completed sessions',.18)]:
        fig.text(.075,y,value,fontsize=54,color=TEAL,weight='bold');fig.text(.075,y-.045,label,fontsize=18,color=GRAY)
    fig.text(.9,.045,'03 / 04',fontsize=12,color=GRAY,ha='right');social_save(fig,'03-validation',pdf)
    fig=canvas();fig.text(.075,.79,'Measured results.\nOpen questions.',fontsize=43,weight='bold',linespacing=1.3)
    items=[('What changed','Blocking, packed weights, patch tiling,\nexact bias folding and bounded requantization.'),('What is public','Rust generator, reusable no_std kernels,\nraw logs, audits and reproducible charts.'),('What remains open','Intermittent QEMU stalls; broader model,\nboard and power-cycle validation.')]
    for (label,body),y in zip(items,[.58,.39,.20]):
        fig.text(.075,y,label,fontsize=20,color=TEAL,weight='bold');fig.text(.075,y-.075,body,fontsize=18,color=GRAY,linespacing=1.6)
    fig.text(.9,.045,'04 / 04',fontsize=12,color=GRAY,ha='right');social_save(fig,'04-scope',pdf)

fig=canvas(12,6.3)
fig.text(.075,.69,'Faster INT8 inference\non scalar RISC-V.',fontsize=37,weight='bold',linespacing=1.25)
fig.text(.73,.54,f"{hw['equal_model_geomean']:.2f}×",fontsize=65,color=TEAL,weight='bold',ha='center')
fig.text(.73,.43,'two-model geometric mean',fontsize=12,color=GRAY,ha='center')
fig.text(.075,.29,'Rust kernels · ESP32-C3 · fastest tested C controls',fontsize=17,color=GRAY)
fig.text(.075,.19,'3 hard resets   /   480 timed inferences   /   0 mismatches',fontsize=14,color=TEAL)
fig.text(.075,.105,'One board; previously used fixtures. QEMU stalls remain undiagnosed.',fontsize=10,color=GRAY)
social_save(fig,'linkedin-cover')
# Wide repository banner, separate from data charts.
fig=plt.figure(figsize=(14,4.2),dpi=120)
fig.text(.05,.78,'SCALAR RV32IMC   /   NO_STD RUST   /   MEASURED',color=TEAL,fontsize=13,weight='bold')
fig.text(.05,.49,'riscv-int8-autotune',fontsize=41,weight='bold')
fig.text(.05,.28,'Generate kernels. Check every tensor. Measure the trade-offs.',fontsize=17,color=GRAY)
fig.text(.05,.10,'2 pretrained models    ·    3 physical reset sessions    ·    open benchmark evidence',fontsize=12,color=BLUE)
fig.savefig(OUT/'cover.svg');fig.savefig(OUT/'cover.png',dpi=180);plt.close(fig)
(OUT/'data-provenance.json').write_text(json.dumps({'inputs':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in inputs},'generator':'scripts/presentation.py','measurement_policy':'No synthetic measurements, inferred uncertainty, or hardware claims from emulator counts. Social 1.59x is the rounded equal-model physical speedup.'},indent=2)+'\n')
print(f'Wrote repository charts to {OUT.relative_to(ROOT)} and social assets to {SOCIAL.relative_to(ROOT)}')

# The physical benchmark uses the restrained technical figure.
import subprocess, sys
subprocess.run([sys.executable, ROOT/"scripts/plot_hardware_clean.py"], check=True)
