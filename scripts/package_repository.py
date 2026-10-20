#!/usr/bin/env python3
"""Create a non-destructive, Git-upload-ready source/evidence snapshot."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import stat
import tempfile
import zipfile

ROOT=Path(__file__).resolve().parents[1]
EXCLUDE_DIRS={'.git','target','.tools','.venv','__pycache__','build','upstream-build','dist','artifacts','managed_components','review'}
EXCLUDE_SUFFIXES={'.a','.o','.elf','.bin','.so','.pyc'}
PRIVATE_KEY=re.compile(rb'-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----')

def selected(path):
    rel=path.relative_to(ROOT)
    if any(part in EXCLUDE_DIRS for part in rel.parts):return False
    if path.name in {'.gitmodules','.DS_Store'}:return False
    return path.suffix not in EXCLUDE_SUFFIXES and not path.is_symlink()

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--out',type=Path,default=ROOT/'dist/riscv-int8-autotune-source.zip')
    p.add_argument('--directory',type=Path,help='Also retain the extracted, clean source directory (must not exist)')
    args=p.parse_args();out=args.out.resolve()
    if out.is_relative_to(ROOT) and out.relative_to(ROOT).parts[0]!='dist':raise ValueError('Write archives outside the source tree or in dist/')
    if args.directory and args.directory.exists():raise ValueError('Directory already exists; choose a new path')
    files=sorted(p for p in ROOT.rglob('*') if p.is_file() and selected(p))
    manifest={};total=0
    with tempfile.TemporaryDirectory(prefix='riscv-int8-release-') as td:
        stage=Path(td)/'riscv-int8-autotune';stage.mkdir()
        for src in files:
            data=src.read_bytes();rel=src.relative_to(ROOT)
            if len(data)>=95*1024**2:raise ValueError(f'File too large for normal Git: {rel}')
            if PRIVATE_KEY.search(data):raise ValueError(f'Private-key marker in {rel}; review before packaging')
            dest=stage/rel;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(data)
            mode=0o755 if src.suffix=='.sh' else 0o644;dest.chmod(mode)
            manifest[str(rel)]=dict(bytes=len(data),sha256=hashlib.sha256(data).hexdigest(),mode=oct(mode))
            total+=len(data)
        (stage/'SOURCE_MANIFEST.json').write_text(json.dumps(dict(project='riscv-int8-autotune',version='0.1.0',files=manifest,source_bytes=total,vendored_baselines=True,policy='Complete selected working-tree sources and measurement evidence, not a Git HEAD export. Excludes build directories, compiled artifacts, tool installations and private review material. Raw captures including failures retained.'),indent=2)+'\n')
        out.parent.mkdir(parents=True,exist_ok=True)
        with zipfile.ZipFile(out,'w',zipfile.ZIP_DEFLATED,compresslevel=6) as archive:
            for path in sorted(stage.rglob('*')):
                if path.is_file():archive.write(path,str(path.relative_to(stage.parent)))
        if args.directory:shutil.copytree(stage,args.directory)
    digest=hashlib.sha256(out.read_bytes()).hexdigest()
    Path(str(out)+'.sha256').write_text(f'{digest}  {out.name}\n')
    print(json.dumps(dict(archive=str(out),sha256=digest,archive_bytes=out.stat().st_size,files=len(files)+1,source_bytes=total,directory=str(args.directory) if args.directory else None),indent=2))

if __name__=='__main__':main()
