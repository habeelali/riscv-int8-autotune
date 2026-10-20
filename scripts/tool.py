#!/usr/bin/env python3
"""Practical entry points for workload creation, diagnostics and isolated tuning."""
import argparse
import csv
import hashlib
import html
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
TARGET = 'riscv32imc-unknown-none-elf'
OBJECTIVES = ['min-instructions', 'min-flash', 'budget', 'pareto']

def write_json(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix+'.tmp')
    tmp.write_text(json.dumps(value, indent=2)+'\n')
    tmp.replace(path)

def call(args):
    result = subprocess.run(list(map(str, args)), capture_output=True, text=True, timeout=120)
    if result.returncode:
        raise RuntimeError(f"Command failed: {' '.join(map(str,args))}\n{result.stdout}\n{result.stderr}")
    return result.stdout

def executable():
    return os.environ.get('RISCV_INT8_BIN', str(ROOT/'target/debug/riscv-int8'))

def inspect(path):
    return json.loads(call([executable(), 'inspect', path]))

def doctor(as_json=False):
    # runner configures the same PATH/library settings used by real benchmarks.
    import runner
    checks=[]
    def check(name, fn, fix):
        try:
            detail=fn()
            checks.append(dict(name=name,ok=True,detail=detail))
        except (OSError, RuntimeError, subprocess.SubprocessError, ValueError) as e:
            checks.append(dict(name=name,ok=False,detail=str(e),fix=fix))
    check('Python',lambda:call([sys.executable,'-c','import tomllib; print("Python TOML support available")']).strip(),'Install Python 3.11 or newer.')
    check('Rust',lambda:call(['rustc','--version']).strip(),'Install the pinned Rust toolchain: rustup show.')
    def target():
        path=Path(call(['rustc','--target',TARGET,'--print','target-libdir']).strip())
        if not list(path.glob('libcore-*.rlib')):raise RuntimeError('Rust target libraries missing')
        return TARGET
    check('Rust target',target,f'rustup target add {TARGET}')
    check('C compiler',lambda:call([runner.CC,'--version']).splitlines()[0],'Run bash scripts/setup.sh or set RISCV_CC and RISCV_TOOLS_PREFIX.')
    for command in ['riscv64-unknown-elf-size','riscv64-unknown-elf-objdump','riscv64-unknown-elf-readelf','riscv64-unknown-elf-ar']:
        check(command,lambda c=command:call([c,'--version']).splitlines()[0],'Source scripts/env.sh or set RISCV_TOOLS_PREFIX.')
    check('QEMU',lambda:call([runner.QEMU,'--version']).splitlines()[0],'Run bash scripts/setup.sh or set QEMU.')
    def cpu():
        names=call([runner.QEMU,'-cpu','help'])
        if 'lowrisc-ibex' not in names:raise RuntimeError('QEMU lacks lowrisc-ibex CPU')
        return 'lowrisc-ibex available; instruction-count calibration runs during build'
    check('QEMU CPU',cpu,'Install the pinned QEMU build from scripts/setup.sh.')
    def sources():
        for name in ['baselines/esp-nn/src/convolution/esp_nn_conv_opt.c','baselines/muriscv-nn/Source/ConvolutionFunctions/muriscv_nn_convolve_s8.c']:
            if not (ROOT/name).is_file():raise RuntimeError(f'Missing {name}')
        return 'Both pinned C baseline source trees present'
    check('Baseline sources',sources,'Use the complete source archive or git submodule update --init --recursive.')
    report=dict(ok=all(c['ok'] for c in checks),root=str(ROOT),checks=checks)
    if as_json:print(json.dumps(report,indent=2))
    else:
        print(f'riscv-int8 environment: {ROOT}')
        for c in checks:
            print(f"  {'OK' if c['ok'] else 'FAIL'}  {c['name']}: {c['detail']}")
            if not c['ok']:print('        '+c['fix'])
        print('Ready for QEMU tuning.' if report['ok'] else 'Resolve failed checks before tuning.')
    return report

def init(args):
    path=args.path.resolve()
    if path.exists():raise ValueError(f'Refusing to overwrite {path}')
    name=path.stem
    if not name or not all(c.isascii() and (c.isalnum() or c=='_') for c in name):
        raise ValueError('Use a TOML filename containing ASCII letters, numbers and underscores.')
    template={'conv':'conv3x3','pointwise':'pointwise','depthwise':'depthwise','fc':'fully_connected'}[args.operator]
    text=(ROOT/f'workloads/{template}.toml').read_text()
    text=text.replace(f'name = "{template}"',f'name = "{name}"',1)
    path.parent.mkdir(parents=True,exist_ok=True)
    # Exclusive create protects against accidentally replacing an existing workload.
    with path.open('x') as f:f.write(text)
    print(f'Created {path}\nEdit tensor dimensions and budgets, then:\n  riscv-int8 build {path} --out artifacts/{name}')

def pareto(rows):
    return [v for v in rows if not any(u['instructions']<=v['instructions'] and u['flash_bytes']<=v['flash_bytes'] and (u['instructions']<v['instructions'] or u['flash_bytes']<v['flash_bytes']) for u in rows)]

def render_report(summary, directory):
    best=summary['selected'];base=summary['baselines']; rows=[('ESP-NN',base['esp_nn']),('muRISCV-NN scalar',base['mu_scalar']),('muRISCV-NN corrected',base['mu_fixed']),('Generic Rust',base['generic_rust']),('Selected Rust',best)]
    bars=[];maximum=max(v['instructions'] for _,v in rows)
    for i,(name,row) in enumerate(rows):
        width=490*row['instructions']/maximum;y=20+i*46;color='#32d6ac' if i==4 else '#86aaff'
        bars.append(f'<text x="0" y="{y+17}" fill="#dce5f4" font-size="13">{html.escape(name)}</text><rect x="190" y="{y}" width="{width:.2f}" height="24" rx="4" fill="{color}"/><text x="{200+width:.2f}" y="{y+17}" fill="#dce5f4" font-size="12">{row["instructions"]:,}</text>')
    plot='<svg viewBox="0 0 810 260" role="img" aria-label="Executed QEMU instructions">'+''.join(bars)+'</svg>'
    body=f'''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width"><title>{html.escape(summary['workload'])} — riscv-int8</title>
<style>body{{margin:0;background:#101c31;color:#edf3fc;font:16px system-ui;line-height:1.6}}main{{max-width:960px;margin:60px auto;padding:0 24px}}h1{{font-size:38px}}small,p{{color:#acbdd4}}strong{{color:#39e3b6}}table{{width:100%;border-collapse:collapse}}td,th{{padding:10px;text-align:left;border-bottom:1px solid #30425c}}a{{color:#80a9ff}}svg{{width:100%}}code{{color:#39e3b6}}</style><main><small>RISC-V / RUST / MEASURED</small><h1>{html.escape(summary['workload'])}</h1><p>Selected <code>{html.escape(best['variant'])}</code> using {html.escape(summary['objective'])}.</p><h2><strong>{summary['best_c_over_selected']:.3f}×</strong> C / selected instruction ratio</h2><p>Against the fastest of three tested C implementations. This is an executed guest instruction comparison, not hardware timing. Values below 1 mean a regression.</p>{plot}<h2>Resource accounting</h2><table><tr><th>Metric</th><th>Selected candidate</th></tr><tr><td>Linked benchmark text</td><td>{best['text_bytes']:,} bytes</td></tr><tr><td>Linked benchmark flash</td><td>{best['flash_bytes']:,} bytes</td></tr><tr><td>Static RAM</td><td>{best['static_ram_bytes']:,} bytes</td></tr><tr><td>Reserved stack</td><td>{best['reserved_stack_bytes']:,} bytes</td></tr><tr><td>Explicit stack scratch</td><td>{best['scratch_bytes']:,} bytes</td></tr><tr><td>Observed stack watermark</td><td>{best['stack_watermark_bytes']:,} bytes</td></tr></table><p>Flash includes immutable fixtures and the oracle. Budget checks use static RAM plus the complete reserved stack. Scratch is within that stack. Measurements are for the linked test harness, not isolated deployment size.</p><h2>Use the export</h2><p><code>cargo check --manifest-path export/Cargo.toml --target {TARGET}</code></p><p>See export/README.md for the tensor and quantization contract. Use your own weights and input buffers with the fixed workload geometry.</p><p>{summary['valid_candidates']} correct candidates; {summary['feasible_candidates']} feasible; {summary['rejected_candidates']} rejected. Bounded search does not promise a global optimum. Numerical validation uses one deterministic fixture per specification and three fresh QEMU executions per implementation; it does not replace application tests.</p><p><a href="summary.json">summary.json</a> · <a href="candidates.csv">all candidate measurements</a> · <a href="pareto.json">Pareto frontier</a> · <a href="export/README.md">integration</a></p></main></html>'''
    (directory/'report.html').write_text(body)
    (directory/'instructions.svg').write_text(plot.replace('<svg ','<svg xmlns="http://www.w3.org/2000/svg" ',1))

def export(summary, spec, directory):
    out=directory/'export';out.mkdir(exist_ok=True)
    selected=summary['selected'];source=(directory/'candidates'/selected['source_file']).read_text()
    source=source.replace('#![no_std]','').replace('#[no_mangle]',f'#[export_name = "riscv_int8_{spec["name"]}"]')
    (out/'kernel.rs').write_text(source)
    (out/'lib.rs').write_text('#![no_std]\npub mod kernel;\npub use kernel::run;\n')
    package='riscv-int8-'+spec['name'].replace('_','-')
    (out/'Cargo.toml').write_text(f'[package]\nname = "{package}"\nversion = "0.1.0"\nedition = "2021"\nlicense = "MIT"\n\n[lib]\npath = "lib.rs"\n\n[workspace]\n')
    shutil.copyfile(ROOT/'LICENSE',out/'LICENSE')
    (out/'kernel.h').write_text(f'''/* RV32IMC INT8 kernel; see README.md for lengths, quantization and non-aliasing. */
#ifndef RISCV_INT8_{spec['name'].upper()}_H
#define RISCV_INT8_{spec['name'].upper()}_H
#include <stdint.h>
#ifdef __cplusplus
extern "C" {{
#endif
void riscv_int8_{spec['name']}(const int8_t *input, const int8_t *weights,
    const int32_t *bias, const int32_t *multiplier, const int32_t *shift, int8_t *output);
#ifdef __cplusplus
}}
#endif
#endif
''')
    write_json(out/'manifest.json',dict(schema=1,spec=spec,selected_config=selected['config'],benchmark=selected,metric='QEMU executed guest instructions',portable_source='kernel.rs',source_sha256=hashlib.sha256((out/'kernel.rs').read_bytes()).hexdigest(),tensor_layout='NHWC activations; OHWI conv weights; HWC depthwise weights; row-major [out,in] FC weights',quantization_contract='Signed INT8; INT32 bias and accumulation; nonnegative per-channel Q31 multipliers; shifts -30..0; two-round requantization. All intermediate arithmetic must fit INT32.',scratch_location='stack',baseline_ratio=summary['best_c_over_selected']))
    lengths=summary['inspection']
    (out/'README.md').write_text(f'''# Generated {spec['name']} kernel

A standalone MIT-licensed `no_std` crate with no dependencies or allocation.
Add this directory as a path dependency, then call `{package.replace('-','_')}::run(input, weights, bias, multipliers, shifts, output)`.
The containing firmware supplies startup, a linker layout and panic handling.

```sh
cargo check --manifest-path Cargo.toml --target {TARGET}
```

Geometry is fixed by workload.toml in the parent directory. Inputs and outputs use NHWC; standard convolution weights use OHWI, depthwise HWC and FC [out,in]. Weights are ordinary signed INT8, not packed by this workflow. Runtime weights and quantization arrays may differ from the benchmark fixture, provided this contract holds.

Buffer lengths: input {lengths['input_elements']}, weights {lengths['weights_elements']}, output {lengths['output_elements']}; bias/multiplier/shift each {spec['output_channels']}. Signed INT8 data; INT32 bias and accumulation; per-channel nonnegative Q31 multipliers; shifts -30..0. Two-round requantization, fixed offsets and activation clamp from the workload. Accumulation and every intermediate arithmetic expression must fit INT32. Output must not alias the input or parameter buffers. Safe Rust checks lengths; C callers must satisfy lengths, alignment and non-aliasing themselves.

Explicit stack scratch: {selected['scratch_bytes']} bytes. Measured whole-harness stack watermark: {selected['stack_watermark_bytes']} bytes; it is not a bound on application interrupts or total stack needs. No heap allocation occurs in the kernel. `kernel.h` declares the optional C ABI symbol. Do not link multiple exports with the same workload name into one binary.

The measured C/selected instruction ratio is {summary['best_c_over_selected']:.4f}x. A value below 1 is slower than the fastest tested C. This is QEMU instruction count, not board latency. `manifest.json` retains configuration, exact compiler flags, fixture quantization and measurements. A deterministic test fixture validates the candidate; verify your application tensors as well.
''')

def build(args):
    if args.max_candidates is not None and args.max_candidates<1:raise ValueError('--max-candidates must be positive')
    source=args.path.resolve();description=inspect(source);spec=description['spec']
    # Refuse configurations whose benchmark fixture cannot fit the requested budget
    # before constructing potentially enormous Python tensor lists.
    fixture_flash=description['input_elements']+description['weights_elements']+description['output_elements']+12*spec['output_channels']
    if fixture_flash>spec['flash_budget']:raise ValueError('Immutable benchmark fixture alone exceeds flash_budget; increase it or reduce dimensions.')
    reduction=spec['kernel']**2*(1 if spec['operator']=='depthwise' else spec['input_channels'])
    if reduction*(128+abs(spec['input_offset']))*128+2176>=2**31:raise ValueError('Conservative fixture accumulation bound exceeds INT32')
    report=doctor()
    if not report['ok']:raise ValueError('Environment checks failed')
    out=args.out.resolve()
    if out==ROOT or ROOT.is_relative_to(out):raise ValueError('Output must be a dedicated job directory, not the repository or its ancestor')
    if out.exists() and any(out.iterdir()) and not args.resume:raise ValueError('Output directory is nonempty; choose a new directory or use --resume')
    out.mkdir(parents=True,exist_ok=True)
    identity=dict(schema=1,spec_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),objective=args.objective,max_candidates=args.max_candidates)
    job=out/'job.json'
    if job.exists():
        previous=json.loads(job.read_text())
        if previous['identity']!=identity:raise ValueError('Resume workload/search differs from this job; choose a new output directory')
    elif args.resume and any(out.iterdir()):raise ValueError('Cannot resume a directory without job.json')
    start=time.monotonic();write_json(job,dict(identity=identity,status='running',stage='baseline',environment=report))
    shutil.copyfile(source,out/'workload.toml')
    import runner
    from benchmark_generic import source as generic_source
    runner.RESULTS=out/'measurements';runner.RESULTS.mkdir(exist_ok=True)
    try:
        print('\n[1/4] Validate and measure optimized C before generating candidates.',flush=True)
        c=runner.benchmark(out/'workload.toml','c')
        write_json(runner.RESULTS/'baseline-milestone.json',dict(validated=True,correct=c['correct'],cache_key=c['cache_key'],metric='QEMU executed guest instructions'))
        baselines={'esp_nn':c}
        for label,kind in [('mu_scalar','muriscv'),('mu_fixed','muriscv-fixed-asm')]:
            baselines[label]=runner.benchmark(out/'workload.toml',kind)
        write_json(out/'baselines.json',baselines)
        print('[2/4] Validate and measure generic Rust.',flush=True)
        generic=out/'generic.rs';generic.write_text(generic_source(spec))
        baselines['generic_rust']=runner.benchmark(out/'workload.toml','generic',generic)
        write_json(out/'baselines.json',baselines)
        print('[3/4] Generate, execute and rank distinct scalar candidates.',flush=True)
        candidate_dir=out/'candidates'
        # New sources are generated to a temporary empty directory; resume keeps logs/cache.
        import tempfile
        with tempfile.TemporaryDirectory(prefix='riscv-int8-generate-') as td:
            generated=Path(td)/'sources'
            command=[executable(),'generate',out/'workload.toml',generated]
            if args.max_candidates is not None:command.append(str(args.max_candidates))
            print(call(command).strip())
            current=json.loads((generated/'candidates.json').read_text());candidate_dir.mkdir(exist_ok=True)
            for item in current:shutil.copyfile(generated/item['source'],candidate_dir/item['source'])
        records=[]
        for i,item in enumerate(current):
            try:
                measured=runner.benchmark(out/'workload.toml','generated',candidate_dir/item['source'])
                row=dict(measured,config=item['config'],variant=item['variant'],source_file=item['source'])
                row['feasible']=row['flash_bytes']<=spec['flash_budget'] and row['static_ram_bytes']+row['reserved_stack_bytes']<=spec['sram_budget']
                print(f"  {i+1}/{len(current)} {item['variant']}: {row['instructions']:,} instructions; {'feasible' if row['feasible'] else 'over budget'}",flush=True)
            except (RuntimeError,subprocess.SubprocessError,OSError) as e:
                row=dict(variant=item['variant'],config=item['config'],source_file=item['source'],correct=False,feasible=False,error=str(e))
                print(f"  {i+1}/{len(current)} {item['variant']}: REJECTED",flush=True)
            records.append(row);write_json(out/'candidates.json',records)
        feasible=[r for r in records if r['correct'] and r['feasible']]
        if not feasible:raise ValueError('No correct candidate meets flash/SRAM constraints; all rejections are retained in candidates.json')
        key=(lambda r:(r['flash_bytes'],r['instructions'])) if args.objective=='min-flash' else (lambda r:(r['instructions'],r['flash_bytes']))
        best=min(feasible,key=key);frontier=pareto(feasible)
        best_c=min(baselines[k]['instructions'] for k in ['esp_nn','mu_scalar','mu_fixed'])
        summary=dict(schema=1,workload=spec['name'],inspection=description,objective=args.objective,metric='QEMU executed guest instructions',search_space_candidates=description['search_candidates'],evaluated_candidates=len(records),valid_candidates=sum(bool(r['correct']) for r in records),feasible_candidates=len(feasible),rejected_candidates=sum(not r['correct'] or not r['feasible'] for r in records),search_wall_seconds=time.monotonic()-start,selected=best,baselines=baselines,best_c_over_selected=best_c/best['instructions'],hardware_timing_measured=False)
        write_json(out/'pareto.json',frontier);write_json(out/'summary.json',summary)
        fields=['variant','correct','feasible','instructions','text_bytes','flash_bytes','static_ram_bytes','reserved_stack_bytes','scratch_bytes','error']
        with (out/'candidates.csv').open('w',newline='') as f:
            w=csv.DictWriter(f,fields,extrasaction='ignore');w.writeheader();w.writerows(records)
        print('[4/4] Export reusable no_std crate and measurement report.',flush=True)
        export(summary,spec,out);render_report(summary,out)
        manifest=out/'export/Cargo.toml'
        call(['cargo','check','--manifest-path',manifest,'--target',TARGET,'--locked'] if (out/'export/Cargo.lock').exists() else ['cargo','check','--manifest-path',manifest,'--target',TARGET])
        write_json(job,dict(identity=identity,status='complete',environment=report,selected=best['variant'],search_wall_seconds=time.monotonic()-start))
        print(f"\nReady: {out/'export'}\nReport: {out/'report.html'}\nC/selected instruction ratio: {summary['best_c_over_selected']:.4f}x (not hardware timing)")
    except BaseException as error:
        write_json(job,dict(identity=identity,status='failed',error=str(error),environment=report))
        raise

def main():
    p=argparse.ArgumentParser(description=__doc__)
    commands=p.add_subparsers(dest='command',required=True)
    d=commands.add_parser('doctor',help='Check the actual cross-compilation and QEMU prerequisites');d.add_argument('--json',action='store_true')
    i=commands.add_parser('init',help='Create a workload TOML template');i.add_argument('path',type=Path);i.add_argument('--operator',choices=['conv','pointwise','depthwise','fc'],default='conv')
    b=commands.add_parser('build',help='Measure baselines, tune and export an isolated standalone kernel');b.add_argument('path',type=Path);b.add_argument('--out',type=Path,required=True);b.add_argument('--objective',choices=OBJECTIVES,default='min-instructions');b.add_argument('--max-candidates',type=int,default=24,help='Evaluate an evenly distributed bounded grid (default 24); use --full-search for every candidate');b.add_argument('--full-search',action='store_true');b.add_argument('--resume',action='store_true')
    args=p.parse_args()
    try:
        if args.command=='doctor':return 0 if doctor(args.json)['ok'] else 1
        if args.command=='init':init(args)
        if args.command=='build':
            if args.full_search:args.max_candidates=None
            build(args)
        return 0
    except (ValueError,RuntimeError,OSError,subprocess.SubprocessError,AssertionError) as e:
        print(f'error: {e}',file=sys.stderr);return 1

if __name__=='__main__':raise SystemExit(main())
