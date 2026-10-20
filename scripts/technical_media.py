#!/usr/bin/env python3
"""Five standalone technical figures. Charts derive exclusively from saved measurements."""
from pathlib import Path
import csv,json,math,hashlib
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch,Rectangle,FancyArrowPatch
R=Path(__file__).resolve().parents[1];O=R/'media/technical-set';O.mkdir(parents=True,exist_ok=True)
def load(s):return json.loads((R/s).read_text())
hw=load('results/hardware-v5/physical-audit/comparison.json');audit=load('results/hardware-v5/physical-audit/summary.json');comp=load('results/competitor-comparison.json');flash=load('results/flash/summary.json')
assert audit['valid_captures']==3 and not audit['emulation_only']
PAPER='#F4F1EA';INK='#172D35';GRAY='#62747B';LINE='#D5DED9';TEAL='#007F82';ORANGE='#DD6134';BLUE='#547AAF';PALE='#E4F0ED';WHITE='#FFFFFF';MUTE='#9EADB0'
plt.rcParams.update({'font.family':'DejaVu Sans','text.color':INK,'axes.labelcolor':GRAY,'xtick.color':GRAY,'ytick.color':INK,'axes.spines.top':False,'axes.spines.right':False,'axes.spines.left':False,'axes.spines.bottom':False,'svg.fonttype':'none','pdf.fonttype':42})

def text(ax,x,y,s,size=12,color=INK,weight='normal',ha='left',va='top',mono=False):
 return ax.text(x,y,s,fontsize=size,color=color,weight=weight,ha=ha,va=va,fontfamily='DejaVu Sans Mono' if mono else 'DejaVu Sans',linespacing=1.45)
def box(ax,x,y,w,h,fill=WHITE,edge=LINE,r=.7):
 p=FancyBboxPatch((x,y),w,h,boxstyle=f'round,pad=0,rounding_size={r}',facecolor=fill,edgecolor=edge,lw=.8);ax.add_patch(p);return p
def line(ax,x1,y1,x2,y2,color=LINE,lw=1):ax.plot([x1,x2],[y1,y2],color=color,lw=lw)
def arrow(ax,x1,y1,x2,y2,color=TEAL):ax.add_patch(FancyArrowPatch((x1,y1),(x2,y2),arrowstyle='-|>',mutation_scale=15,color=color,lw=1.5))
def base(n,title,sub,scope):
 f=plt.figure(figsize=(16,12),facecolor=PAPER);a=f.add_axes([0,0,1,1]);a.set(xlim=(0,100),ylim=(0,75));a.axis('off')
 box(a,3,68.3,2.7,2.5,INK,INK,.2);text(a,4.35,70.05,f'{n:02}',11,WHITE,'bold',ha='center')
 text(a,7.2,70.1,'riscv-int8-autotune',12,INK,'bold');text(a,97,70.1,'RV32IMC',10,GRAY,ha='right')
 line(a,3,66.8,97,66.8,INK,.8)
 text(a,3,64.4,title,30,INK,'bold');text(a,3,59.8,sub,12,GRAY)
 line(a,3,5.4,97,5.4,INK,.7);text(a,3,3.95,scope,9,GRAY)
 return f,a
def plotax(f,x,y,w,h):
 ax=f.add_axes([x/100,y/75,w/100,h/75],facecolor=WHITE)
 for s in ax.spines.values():s.set_visible(False)
 ax.tick_params(length=0,labelsize=10);return ax
def save(f,name):
 for ext in ['png','svg']:f.savefig(O/f'{name}.{ext}',dpi=150,facecolor=PAPER)
 plt.close(f)

# 01: all tested hardware controls plus a visual encoding of relative latency.
f,a=base(1,'Inference time','ESP32-C3 · 160 MHz CPU · 40 MHz DIO flash','1 board · 3 hard resets · 4 inputs/model · mean of input medians')
for m,x in [(0,3),(1,51)]:
 model=hw['models'][m];box(a,x,24,46,32)
 text(a,x+2,54,model['model'],17,INK,'bold');text(a,x+2,50.5,['TFLM · 96 × 96 grayscale','MLCommons Tiny · 32 × 32 RGB'][m],10,GRAY)
 ax=plotax(f,x+15,28,27,19)
 vals=[np.mean([next(s['mean_input_median_ms'] for s in c['statistics'] if s['model']==m and s['backend']==b) for c in audit['captures']]) for b in range(4)]
 labels=['ESP-NN','mu scalar','mu corrected','Rust generated']
 ax.barh(range(4),vals,color=[MUTE,MUTE,MUTE,ORANGE],height=.52)
 ax.set_yticks(range(4),labels,fontsize=10);ax.invert_yaxis();ax.set_xlim(0,1120);ax.set_xticks([0,250,500,750,1000]);ax.grid(axis='x',color='#ECF0ED');ax.set_axisbelow(True)
 for i,v in enumerate(vals):ax.text(v+16,i,f'{v:.2f}',va='center',fontsize=10,weight='bold' if i==3 else 'normal',color=ORANGE if i==3 else INK)
 ax.set_xlabel('Complete inference time (ms)',fontsize=10,labelpad=10)
 text(a,x+2,21,f"{model['c_over_rust']:.2f}×",34,ORANGE,'bold')
 text(a,x+17,20.5,'vs best tested C',11,INK,'bold');text(a,x+17,18,f"{100*(1-model['rust_ms']/model['best_tested_c_ms']):.1f}% less inference time",11,GRAY)
 # 100 cells show normalized latency, explicitly rounded, not run observations.
 remaining=round(100*model['rust_ms']/model['best_tested_c_ms'])
 for i in range(100):
  xx=x+2+(i%25)*1.64;yy=11.4+(3-i//25)*1.28
  box(a,xx,yy,1.27,.85,TEAL if i<remaining else '#DDDAD2',TEAL if i<remaining else '#DDDAD2',.12)
 text(a,x+2,9.8,f'Rust time / C time: {remaining}% (rounded)',9,GRAY)
text(a,97,62.2,f"{hw['equal_model_geomean']:.2f}×",22,TEAL,'bold',ha='right');text(a,97,58.9,'two-model geometric mean',9,GRAY,ha='right')
save(f,'01-hardware')

# 02: actual microtile dataflow, not fabricated measurements.
f,a=base(2,'Kernel design','INT8 patch tiles · packed weights · register reuse','ResNet kernel · 4 × 2 microtile · schematic')
box(a,3,30,94,26)
text(a,5,54,'4 output positions × 2 channels',17,INK,'bold')
text(a,6,49.5,'INPUT PATCH',10,TEAL,'bold');text(a,35,49.5,'PACKED WEIGHTS',10,BLUE,'bold');text(a,68,49.5,'ACCUMULATORS',10,ORANGE,'bold')
# Each reduction step consumes four input values and two weights.
for row in range(5):
 for col in range(4):
  xx=6+col*4.6;yy=34+row*2.65;highlight=row==2
  box(a,xx,yy,3.8,2.1,TEAL if highlight else PALE,TEAL if highlight else '#C8DCD7',.22)
  text(a,xx+1.9,yy+1.6,f'x{col}' if highlight else '·',10,WHITE if highlight else TEAL,ha='center',mono=True)
for row in range(5):
 for col in range(2):
  xx=35+col*5;yy=34+row*2.65;highlight=row==2
  box(a,xx,yy,4.2,2.1,BLUE if highlight else '#E9EEF5',BLUE if highlight else '#D2DDE9',.22)
  text(a,xx+2.1,yy+1.6,f'w{col}' if highlight else '·',10,WHITE if highlight else BLUE,ha='center',mono=True)
for row in range(4):
 for col in range(2):
  xx=68+col*11;yy=34+row*3
  box(a,xx,yy,9.5,2.4,'#FFF0E7','#EDC9B8',.25)
  text(a,xx+4.75,yy+1.85,f'a{3-row}{col}',11,ORANGE,'bold',ha='center',mono=True)
arrow(a,25,40.3,32,40.3);arrow(a,47,40.3,64,40.3,BLUE)
text(a,27.5,44,'reuse',9,GRAY,ha='center');text(a,55.5,44,'outer product',9,GRAY,ha='center')
text(a,6,32.5,'Reduction K × 4 positions',9,GRAY);text(a,35,32.5,'K × 2 output channels',9,GRAY);text(a,68,32.5,'8 scalar accumulators',9,GRAY)
# Two substantial lower panels.
box(a,3,8,46,19);box(a,51,8,46,19,INK,INK)
text(a,5,25,'Per reduction step',15,INK,'bold')
text(a,5,20,'4 inputs + 2 weights',21,TEAL,'bold');text(a,5,15.8,'→ 8 multiply–accumulate updates',13,INK)

text(a,53,25,'Quantization and memory',15,WHITE,'bold')
text(a,53,20.8,'• INT8 zero-point padding\n• Exact bias folding and requantization\n• Tile scratch ≤ 9,216 B\n• Task stack: 16 KiB',11,'#D6E4E5')
save(f,'02-kernel-dataflow')

# 03: concrete CLI product, with outputs and sequential measurement gates.
f,a=base(3,'Tool workflow','Workload TOML → measured kernel → no_std export','Operator kernels · runtime weights · Rust and C interfaces')
box(a,3,32,44,24,INK,INK);text(a,5,54,'CLI',10,'#93ACB1','bold')
text(a,5,49.8,'$ riscv-int8 doctor\n$ riscv-int8 init conv.toml --operator conv\n$ riscv-int8 inspect conv.toml\n$ riscv-int8 build conv.toml \\\n    --out artifacts/conv --max-candidates 24',10,'#DCEBE9',mono=True)

# Gate pipeline: 6 real steps, not performance claims.
steps=[('01','BASELINE','ESP-NN + muRISCV-NN'),('02','REFERENCE','Generic Rust, same tensors'),('03','GENERATE','Distinct scalar loop bodies'),('04','EXECUTE','RV32IMC QEMU + oracle'),('05','SELECT','Correct + within budgets'),('06','EXPORT','Crate + evidence + report')]
for idx,(num,label,desc) in enumerate(steps):
 y=52-idx*7
 box(a,51,y-2,46,5.7,WHITE,LINE)
 box(a,52,y-1,3.8,3.8,ORANGE if idx==5 else PALE,ORANGE if idx==5 else PALE,.3)
 text(a,53.9,y+1.8,num,11,WHITE if idx==5 else TEAL,'bold',ha='center')
 text(a,58,y+2,label,11,INK,'bold');text(a,58,y-.1,desc,10,GRAY)
 if idx<5:arrow(a,74,y-2.2,74,y-3.1)
box(a,3,8,44,21,WHITE,LINE)
text(a,5,27,'OUTPUT',10,TEAL,'bold')
text(a,5,23.4,'export/\n├── Cargo.toml + lib.rs   no_std crate\n├── kernel.rs + kernel.h Rust + C ABI\n├── manifest.json        exact contract\n└── README.md + LICENSE',11,INK,mono=True)
text(a,51,10.5,'Cached and resumable searches',10,TEAL,'bold')
save(f,'03-tool-workflow')

# 04: two clearly separated archived studies and actual values.
f,a=base(4,'Instructions and flash','QEMU operator benchmarks · person-model flash measurements','Guest instructions, not cycles · flash figures: earlier person-model build')
box(a,3,25,94,31)
text(a,5,54,'INSTRUCTION REDUCTION',11,TEAL,'bold');text(a,95,54,'C / Rust',10,GRAY,ha='right')
ax=plotax(f,19,30,72,18)
rows=[next(r for r in comp['rows'] if r['workload']==n) for n in ['conv3x3','pointwise','depthwise','fully_connected']]
y=np.arange(4)
for off,key,label,color in [(-.18,'improvement_vs_esp_nn','ESP-NN',BLUE),(.18,'improvement_vs_best_muriscv','muRISCV-NN (best tested)',TEAL)]:
 vals=[r[key] for r in rows];ax.barh(y+off,vals,height=.3,color=color,label=label)
 for yy,v in zip(y+off,vals):ax.text(v+.025,yy,f'{v:.3f}×',va='center',fontsize=11,color=color,weight='bold')
ax.axvline(1,color=GRAY,lw=1,ls='--');ax.set_yticks(y,['3×3 conv','1×1 conv','Depthwise','Fully connected']);ax.invert_yaxis();ax.set_xlim(0,2.65);ax.set_xticks([0,.5,1,1.5,2,2.5]);ax.grid(axis='x',color='#EDF0ED');ax.set_axisbelow(True)
ax.legend(loc='upper right',bbox_to_anchor=(1,1.25),ncol=2,frameon=False,fontsize=10);ax.set_xlabel('C instructions / Rust instructions',fontsize=10,labelpad=10)

box(a,3,8,46,14);box(a,51,8,46,14)
text(a,5,20,'GENERATED CODE',10,TEAL,'bold');text(a,5,17,'63,858 → 48,138 B',20,INK,'bold');text(a,5,12.9,f"−{100*flash['code_reduction']:.1f}%",19,ORANGE,'bold');text(a,20,12.2,'Same instruction counts',10,GRAY)
text(a,53,20,'DEPLOYMENT IMAGE',10,TEAL,'bold');text(a,53,17,'302,832 → 286,536 B',20,INK,'bold');text(a,53,12.9,f"−{100*flash['deploy_image_reduction']:.1f}%",19,ORANGE,'bold');text(a,67,12.2,'Person-model build',10,GRAY)
save(f,'04-instructions-flash')

# 05: actual every-run plots and explicit validation structure.
f,a=base(5,'Validation','3 hard resets · 5 repeats per input/backend','1 board · 4 inputs/model · ESP32-C3 QEMU stalls unresolved')
for x,value,label in [(3,'2,256','intermediate tensor checks'),(35,'480','timed complete inferences'),(67,'0','numerical mismatches')]:
 box(a,x,45,30,11,WHITE,LINE);text(a,x+2,54.5,value,28,ORANGE if value=='0' else INK,'bold');text(a,x+2,48.8,label,10,GRAY)
with (R/'results/hardware-v5/physical-audit/runs.csv').open() as file:raw=list(csv.DictReader(file))
for m,x in [(0,3),(1,51)]:
 box(a,x,16,46,26)
 text(a,x+2,40.5,hw['models'][m]['model']+' / Rust',14,INK,'bold')
 ax=plotax(f,x+7,21,35,15)
 for boot,color,off in [(1,TEAL,-.17),(2,ORANGE,0),(3,BLUE,.17)]:
  for sample in range(4):
   vals=[float(r['inference_ms']) for r in raw if int(r['model'])==m and int(r['backend'])==3 and int(r['sample'])==sample and Path(r['capture']).name==f'boot-{boot}']
   ax.scatter(sample+off+np.linspace(-.035,.035,len(vals)),vals,s=20,color=color,alpha=.9,label=f'Boot {boot}' if sample==0 else None)
 ax.ticklabel_format(axis='y',useOffset=False);ax.set_xticks(range(4),['Input 0','Input 1','Input 2','Input 3']);ax.tick_params(labelsize=9);ax.grid(axis='y',color='#E7ECE9');ax.set_axisbelow(True);ax.set_ylabel('Inference time (ms)',fontsize=9);ax.legend(frameon=False,ncol=3,fontsize=8,loc='upper right',bbox_to_anchor=(1,1.14))
 text(a,x+2,18.2,'All Rust runs · zoomed y-axis',9,GRAY)
text(a,3,12.8,'CHECKS',10,TEAL,'bold')
text(a,3,10,'Capture hashes  →  session challenges  →  tensor checks  →  clock calibration',10,INK)
save(f,'05-validation')
files=['results/hardware-v5/physical-audit/comparison.json','results/hardware-v5/physical-audit/summary.json','results/hardware-v5/physical-audit/runs.csv','results/competitor-comparison.json','results/flash/summary.json']
(O/'data-sources.json').write_text(json.dumps({p:hashlib.sha256((R/p).read_bytes()).hexdigest() for p in files},indent=2)+'\n')
print(O)
