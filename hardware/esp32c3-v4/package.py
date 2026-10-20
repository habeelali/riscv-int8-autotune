#!/usr/bin/env python3
"""Package the exact emulator-validated v4 binary and portable capture tools."""
import argparse,hashlib,json,os,shutil,subprocess,sys,zipfile
from pathlib import Path
H=Path(__file__).resolve().parent;R=H.parents[1];B=H/'build'
p=argparse.ArgumentParser(description=__doc__);p.add_argument('output',type=Path);a=p.parse_args();out=a.output.resolve();out.mkdir(parents=True,exist_ok=True)
image=out/'esp32c3-generalize-v4.bin'
subprocess.run([sys.executable,'-m','esptool','--chip','esp32c3','merge_bin','-o',image,'--flash_mode','dio','--flash_freq','40m','--flash_size','4MB','--fill-flash-size','4MB','0x0',B/'bootloader/bootloader.bin','0x8000',B/'partition_table/partition-table.bin','0x10000',B/'riscv_int8_generalize.bin'],check=True)
sha=hashlib.sha256(image.read_bytes()).hexdigest();test=json.loads((R/'results/hardware-v4/emulator-functional-check.json').read_text());prov=json.loads((H/'main/generated/provenance.json').read_text());assert test['image_sha256']==sha and test['functional_pass'] and test['build_id']==prov['build_id']
manifest=dict(schema=4,chip='ESP32-C3',minimum_flash_bytes=4194304,flash_offset='0x0',image=image.name,image_bytes=image.stat().st_size,image_sha256=sha,build_id=prov['build_id'],cpu_hz=160000000,flash_mode='DIO',flash_frequency_hz=40000000,baudrate=115200,expected_checks=1880,expected_calibrations=3,expected_runs=400,expected_profiles=96,models=['person','cifar10_resnet'],backends=__import__('selection').BACKENDS,physical_v4_tested=False,target_test=test)
(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n');(out/'firmware.sha256').write_text(sha+'  '+image.name+'\n')
for n in ['collect.py','audit.py','selection.py','README.md','AGENT_PROMPT.md','sdkconfig','partitions.csv']:shutil.copyfile(H/n,out/n)
for n in ['oracle.json','provenance.json']:shutil.copyfile(H/'main/generated'/n,out/n)
shutil.copyfile(B/'riscv_int8_generalize.elf',out/'firmware.elf');shutil.copyfile(B/'bootloader/bootloader.elf',out/'bootloader.elf')
shutil.copyfile(R/'results/hardware-v4/emulator-functional-check.json',out/'emulator-functional-check.json')
shutil.copyfile(R/'models/cifar10-resnet/source.json',out/'resnet-source.json');shutil.copyfile(R/'models/person-detection/source.json',out/'person-source.json')
shutil.copyfile(R/'THIRD_PARTY.md',out/'THIRD_PARTY.md');(out/'requirements.txt').write_text('esptool==4.12.0\npyserial==3.5\n')
idf=Path(os.environ.get('IDF_PATH','/workspace/esp-idf-v5.4.2'));licenses={'PROJECT-MIT':R/'LICENSE','ESP-IDF':idf/'LICENSE','FreeRTOS':idf/'components/freertos/FreeRTOS-Kernel/LICENSE.md','ESP-NN':R/'baselines/esp-nn/LICENSE','muRISCV-NN':R/'baselines/muriscv-nn/LICENSE','TFLM':R/'models/person-detection/LICENSE.tflite-micro','MLCommons-Tiny':R/'models/cifar10-resnet/LICENSE.mlcommons'}
(out/'LICENSES').mkdir(exist_ok=True)
for name,f in licenses.items():shutil.copyfile(f,out/'LICENSES'/(name+'.txt'))
archive=out.with_suffix('.zip')
with zipfile.ZipFile(archive,'w',zipfile.ZIP_DEFLATED,compresslevel=9) as z:
 for f in sorted(out.rglob('*')):
  if f.is_file():z.write(f,f.relative_to(out))
print(json.dumps(dict(bundle=str(archive),bytes=archive.stat().st_size,sha256=sha,build_id=prov['build_id']),indent=2))
