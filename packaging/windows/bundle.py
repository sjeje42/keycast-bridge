"""Collect the PE dependency closure from MSYS2 UCRT64, not the build toolchain."""
import os, re, shutil, subprocess
assert os.name == "posix", "Run with /usr/bin/python inside MSYS2 UCRT64"
from pathlib import Path
prefix = Path(os.environ.get('MINGW_PREFIX', '/ucrt64'))
out = Path('dist/keycast-bridge-windows-x64')
out.mkdir(parents=True, exist_ok=True)
queue=[]
for name in ['keycast-bridge.exe', 'keycast-bridge-demo.exe']:
    dst=out/name
    shutil.copy2(Path('target/release')/name,dst)
    queue.append(dst)
seen=set()
packages=set()
# GTK dynamically loads pixbuf loaders: copy and inspect those too.
loaders=prefix/'lib/gdk-pixbuf-2.0'
if loaders.exists():
    shutil.copytree(loaders,out/'lib/gdk-pixbuf-2.0',dirs_exist_ok=True)
    queue.extend((out/'lib/gdk-pixbuf-2.0').rglob('*.dll'))
# Module caches must resolve inside the extracted archive, not the build prefix.
for cache in (out/'lib/gdk-pixbuf-2.0').rglob('loaders.cache'):
    lines=[]
    for line in cache.read_text(encoding='utf-8').splitlines():
        match=re.fullmatch(r'"([^"]+\.dll)"',line,re.IGNORECASE)
        if match:
            name=match.group(1).replace('\\\\','/').split('/')[-1]
            candidates=list((out/'lib/gdk-pixbuf-2.0').rglob(name))
            if len(candidates)!=1: raise RuntimeError('Unresolved pixbuf loader: '+name)
            line='"'+candidates[0].relative_to(out).as_posix()+'"'
        lines.append(line)
    cache.write_text('\n'.join(lines)+'\n',encoding='utf-8')
while queue:
    path=queue.pop()
    imports=subprocess.check_output(['objdump','-p',str(path)],text=True)
    for name in re.findall(r'DLL Name:\s*(\S+)',imports):
        key=name.lower()
        if key in seen: continue
        seen.add(key)
        source=prefix/'bin'/name
        if not source.exists():
            # Windows system DLLs are provided by the OS.
            system=Path(subprocess.check_output(['cygpath','-u',os.environ.get('SYSTEMROOT','C:/Windows')],text=True).strip())/'System32'/name
            if not system.exists() and not key.startswith(('api-ms-win-', 'ext-ms-win-')):
                raise RuntimeError('Missing dependency: '+name)
            continue
        dest=out/name
        shutil.copy2(source,dest)
        queue.append(dest)
        packages.add(subprocess.check_output(['pacman','-Qqo',str(source)],text=True).strip())
for rel in ['share/glib-2.0/schemas','share/icons/Adwaita','share/icons/hicolor','share/locale']:
    src=prefix/rel
    if src.exists(): shutil.copytree(src,out/rel,dirs_exist_ok=True)
licenses=out/'licenses'
licenses.mkdir(exist_ok=True)
shutil.copy2('LICENSE',licenses/'Keycast-Bridge.txt')
if (prefix/'share/licenses').exists(): shutil.copytree(prefix/'share/licenses',licenses/'MSYS2',dirs_exist_ok=True)
manifest=[]
for package in sorted(packages):
    manifest.append(subprocess.check_output(['pacman','-Qi',package],text=True))
(licenses/'MSYS2-packages.txt').write_text('\n'.join(manifest),encoding='utf-8')
shutil.copy2('docs/WINDOWS.md',out/'README.md')
shutil.copytree('docs/guide',out/'guide',dirs_exist_ok=True)
shutil.copytree('docs/pdf',out/'guide/pdf',dirs_exist_ok=True)
for language in ['en', 'fr']:
    shutil.copy2(f'docs/USER_GUIDE.{language}.md',out/f'USER_GUIDE.{language}.md')
print('Portable DLL closure:',len(seen),'dependencies')
