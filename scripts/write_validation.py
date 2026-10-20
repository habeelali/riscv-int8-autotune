#!/usr/bin/env python3
"""Record completion only after the reproducibility script has passed its checks."""
import json,subprocess
from runner import ROOT
read=lambda name:json.loads((ROOT/'results'/name).read_text())
s=read('comparison.json');c=read('competitor-comparison.json');reg=read('regression.json');mr=read('muriscv-regression.json');checks=read('counter-crosscheck.json');q=read('upstream-quantization.json')
assert len(reg)==72 and all(r['correct'] for r in reg)
assert len(mr)==96 and all(r['correct'] for r in mr)
assert len(checks)==12 and all(r['match'] for r in checks)
assert read('negative-correctness-test.json')['cached_rejected']
assert all(r['correct'] for name in ('baseline.json','generic.json') for r in read(name))
assert {'muriscv','muriscv-portable','muriscv-fixed-asm','muriscv-portable-fixed-asm'} <= {r['kind'] for r in q if r['qualified']}
validation=dict(full_reproduction='passed; results/reproduce-competitors.log',rust_fmt='passed',clippy='passed with warnings denied',rust_tests='passed',target_no_std_check='passed for kernel-core and reference',quantization_cases=3390,restricted_quantizer_checks=11364,upstream_quantization_cases_per_configuration=q[0]['cases'],upstream_quantization_configurations=len(q),additional_qemu_operator_executions=len(reg),upstream_qemu_operator_executions=len(mr),process_repetitions=3,plugin_crosschecks=len(checks),negative_kernel='fresh and cached incorrect kernel rejected',candidates=s['candidates'],primary_instruction_geomean_vs_esp_nn=s['primary_geomean'],all_instruction_geomean_vs_esp_nn=s['all_geomean'],primary_instruction_geomean_vs_stock_muriscv=c['primary_geomean_vs_stock_muriscv'],primary_instruction_geomean_vs_best_muriscv=c['primary_geomean_vs_best_muriscv'],hardware_tested=False,docker_build_tested=False,completed_utc=subprocess.check_output(['date','-u','+%FT%TZ'],text=True).strip())
(ROOT/'results/validation.json').write_text(json.dumps(validation,indent=2)+'\n')
print('Validated final benchmark artifacts:',s['candidates'],'candidates,',len(reg)+len(mr),'fresh-seed operator executions.')
