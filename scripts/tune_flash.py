#!/usr/bin/env python3
"""Bounded full-model flash search with strict per-input/per-layer non-regression."""
import hashlib
import json
from pathlib import Path
import shutil
import time

from benchmark_model import benchmark
from model_import import load_model
from runner import ROOT, RFLAGS, run

DEST = ROOT/'results/flash'
REFERENCE = DEST/'reference'
EXPORT = ROOT/'generated/person-detection/flash'


def constraints(candidate, reference):
    """Fail closed on changed sample/layer identities or any measured regression."""
    failures = []
    if not candidate['correct']:
        failures.append('incorrect')
    for key in ('text_bytes','static_ram_bytes'):
        if candidate[key] > reference[key]:
            failures.append(key)
    if candidate['text_bytes']+candidate['rodata_bytes'] >= reference['text_bytes']+reference['rodata_bytes']:
        failures.append('no read-only image reduction')
    if len(candidate['samples']) != len(reference['samples']) or len(candidate['layers']) != len(reference['layers']):
        return failures+['different validation corpus']
    for a,b in zip(candidate['samples'],reference['samples']):
        if a['name'] != b['name'] or a['output'] != b['output'] or a['mismatches']:
            failures.append('sample correctness/identity')
        if a['instructions'] > b['instructions']:
            failures.append('whole-model instructions: '+a['name'])
        if a['stack_watermark_bytes'] > b['stack_watermark_bytes']:
            failures.append('stack: '+a['name'])
    for a,b in zip(candidate['layers'],reference['layers']):
        if (a['sample'],a['layer']) != (b['sample'],b['layer']) or a['mismatches']:
            failures.append('layer correctness/identity')
        if a['instructions'] > b['instructions']:
            failures.append(f'layer instructions: {a["sample"]}/{a["layer"]}')
    return failures


def main():
    started = time.monotonic()
    DEST.mkdir(exist_ok=True)
    if not REFERENCE.exists():
        REFERENCE.mkdir()
        shutil.copyfile(ROOT/'results/model/person-detection/results.json',REFERENCE/'model-results.json')
        for name in ('kernels.rs','manifest.json','selection.json'):
            shutil.copyfile(ROOT/'generated/person-detection'/name,REFERENCE/name)
    manifest = load_model(ROOT/'results/model/person-detection')
    assert manifest == json.loads((REFERENCE/'manifest.json').read_text()), 'Model/reference changed; make a new explicit baseline'
    original = next(r for r in json.loads((REFERENCE/'model-results.json').read_text()) if r['kind']=='generated-selected')
    replay = benchmark(manifest,'flash-reference',source_override=REFERENCE/'kernels.rs')
    assert replay['text_bytes'] == original['text_bytes']
    assert all(a['instructions']==b['instructions'] for a,b in zip(replay['samples'],original['samples']))
    assert replay['correct']
    (DEST/'replayed-reference.json').write_text(json.dumps(replay,indent=2)+'\n')
    rows = []
    sources = DEST/'sources';sources.mkdir(exist_ok=True)
    for mode in ('dedup','outline','dedup-outline'):
        source = sources/(mode+'.rs')
        run(['cargo','run','--quiet','--locked','-p','kernel-generator','--example','flash_model',
             '--',REFERENCE/'kernels.rs',source,mode],cwd=ROOT)
        for opt in ('3','s','z'):
            flags = [f'opt-level={opt}' if x=='opt-level=3' else x for x in RFLAGS]
            name = f'{mode}-O{opt}'
            result = benchmark(manifest,'flash-'+name,source_override=source,rustflags=flags)
            result['variant'] = name
            result['source'] = str(source.relative_to(ROOT))
            result['rejection_reasons'] = constraints(result,replay)
            result['eligible'] = not result['rejection_reasons']
            rows.append(result)
            (DEST/'candidates.json').write_text(json.dumps(rows,indent=2)+'\n')
            print(name,result['text_bytes'],'text bytes;',result['mean_instructions'],'instructions;',
                  'ELIGIBLE' if result['eligible'] else 'REJECTED: '+', '.join(result['rejection_reasons'][:3]),flush=True)
    eligible = [r for r in rows if r['eligible']]
    if not eligible:
        raise RuntimeError('No flash candidate satisfies strict non-regression; original export retained')
    winner = min(eligible,key=lambda r:(r['text_bytes']+r['rodata_bytes'],r['text_bytes'],r['mean_instructions']))
    summary = dict(reference=replay,original_reference_key=original['cache_key'],best=winner,
                   source_sha256=hashlib.sha256((ROOT/winner['source']).read_bytes()).hexdigest(),
                   objective='minimum text+rodata; no input/layer instruction, static RAM or stack regression',
                   search_wall_seconds=time.monotonic()-started,candidates=len(rows))
    (DEST/'best.json').write_text(json.dumps(summary,indent=2)+'\n')
    EXPORT.mkdir(exist_ok=True)
    shutil.copyfile(ROOT/winner['source'],EXPORT/'kernels.rs')
    for name in ('manifest.json','selection.json'):
        shutil.copyfile(REFERENCE/name,EXPORT/name)
    shutil.copyfile(DEST/'best.json',EXPORT/'selection-evidence.json')
    print('Selected',winner['variant'],'with zero measured regressions',flush=True)


if __name__ == '__main__':
    main()
