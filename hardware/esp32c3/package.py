#!/usr/bin/env python3
"""Package the built v2 cache diagnostic, including the exact image tested in QEMU."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import zipfile

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
p=argparse.ArgumentParser(description=__doc__)
p.add_argument('output',type=Path)
args=p.parse_args()
out=args.output.resolve();out.mkdir(parents=True,exist_ok=True)
build=HERE/'build'
image=out/'esp32c3-int8-cache-v2.bin'
subprocess.run([sys.executable,'-m','esptool','--chip','esp32c3','merge_bin','-o',str(image),
    '--flash_mode','dio','--flash_freq','40m','--flash_size','4MB','--fill-flash-size','4MB',
    '0x0',str(build/'bootloader/bootloader.bin'),'0x8000',str(build/'partition_table/partition-table.bin'),
    '0x10000',str(build/'riscv_int8_hardware_bench.bin')],check=True)
sha=hashlib.sha256(image.read_bytes()).hexdigest()
functional=json.loads((ROOT/'results/hardware-cache/emulator-functional-check.json').read_text())
provenance=json.loads((HERE/'main/generated/provenance.json').read_text())
assert functional['functional_pass'] and functional['build_id']==provenance['build_id']
tested=ROOT/'results/hardware-cache/tested-image.sha256'
assert tested.read_text().split()[0]==sha,'Packaged image differs from the QEMU-tested image'
manifest=dict(chip='ESP32-C3',minimum_flash_bytes=4194304,flash_offset='0x0',image=image.name,
    image_bytes=image.stat().st_size,image_sha256=sha,build_id=provenance['build_id'],cpu_hz=160000000,
    flash_mode='DIO',flash_frequency_hz=40000000,uart_baudrate=115200,
    expected_layer_records=1116,expected_run_records=252,expected_end_marker='HW_DONE',
    backends=['esp_nn_native','mu_scalar','mu_portable','mu_fixed','mu_portable_fixed',
              'rust_compact','rust_candidate','rust_cache4','rust_cache2'],
    target_test=functional,hardware_performance_for_v2='UNMEASURED',
    diagnostic_flash_size_is_not_single_model_size=True)
(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
(out/'firmware.sha256').write_text(sha+'  '+image.name+'\n')
for name in ['capture.py','summarize.py','AGENT_PROMPT.md','README.md','sdkconfig','partitions.csv']:
    shutil.copyfile(HERE/name,out/name)
shutil.copyfile(HERE/'main/generated/provenance.json',out/'provenance.json')
shutil.copyfile(build/'riscv_int8_hardware_bench.elf',out/'firmware.elf')
shutil.copyfile(build/'bootloader/bootloader.elf',out/'bootloader.elf')
shutil.copyfile(ROOT/'results/hardware-cache/emulator-functional-check.json',out/'emulator-functional-check.json')
shutil.copyfile(ROOT/'THIRD_PARTY.md',out/'THIRD_PARTY.md')
shutil.copyfile(ROOT/'models/person-detection/source.json',out/'model-source.json')
(out/'requirements.txt').write_text('esptool==4.12.0\npyserial==3.5\n')
idf=Path(os.environ.get('IDF_PATH','/workspace/esp-idf-v5.4.2'))
licenses={'PROJECT-MIT':ROOT/'LICENSE','ESP-IDF':idf/'LICENSE',
 'FreeRTOS':idf/'components/freertos/FreeRTOS-Kernel/LICENSE.md',
 'ESP-NN':ROOT/'baselines/esp-nn/LICENSE','muRISCV-NN':ROOT/'baselines/muriscv-nn/LICENSE',
 'TFLM':ROOT/'models/person-detection/LICENSE.tflite-micro'}
(out/'LICENSES').mkdir(exist_ok=True)
for label,src in licenses.items():shutil.copyfile(src,out/'LICENSES'/(label+'.txt'))
archive=out.with_suffix('.zip')
with zipfile.ZipFile(archive,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=9) as z:
    for path in sorted(out.rglob('*')):
        if path.is_file():z.write(path,path.relative_to(out))
print(json.dumps(dict(bundle=str(archive),bytes=archive.stat().st_size,image_sha256=sha,build_id=provenance['build_id']),indent=2))
