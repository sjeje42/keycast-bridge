#!/usr/bin/env python3
"""Check active text docs/manifests against Cargo; historical releases are excluded."""
import json
import re
import sys
import tomllib
from pathlib import Path

root = Path(__file__).resolve().parent.parent
version = tomllib.loads((root / 'Cargo.toml').read_text())['package']['version']
errors = []


def expect(label, actual):
    if actual != version:
        errors.append(f'{label}: {actual!r}, expected {version!r}')


lock = tomllib.loads((root / 'Cargo.lock').read_text())
expect('Cargo.lock', next(p['version'] for p in lock['package'] if p['name'] == 'keycast-bridge'))
web = json.loads((root / 'web/package.json').read_text())
web_lock = json.loads((root / 'web/package-lock.json').read_text())
expect('web/package.json', web['version'])
expect('web/package-lock.json', web_lock['version'])
expect('web/package-lock.json root package', web_lock['packages']['']['version'])

# PDF guides and CHANGELOG are historical snapshots, reviewed independently.
current_docs = [
    'README.md', 'README.fr.md', 'packaging/RELEASE.md',
    'docs/USER_GUIDE.en.md', 'docs/USER_GUIDE.fr.md',
    'docs/guide/en.html', 'docs/guide/fr.html',
    'docs/WINDOWS.md', 'docs/VALIDATION.md', 'docs/RELEASING.md',
]
deb_version = version.replace('-', '~', 1) + '-1'
assets = {
    f'keycast-bridge_{version}_windows-x64.zip',
    f'keycast-bridge_{deb_version.replace("~", ".")}_amd64.deb',
}
for name in current_docs:
    text = (root / name).read_text()
    if version not in text:
        errors.append(f'{name}: missing current version {version}')
    for found in re.findall(r'\b\d+\.\d+\.\d+-(?:alpha|beta|rc)\.\d+\b', text):
        expect(name, found)
    for asset in re.findall(r'keycast-bridge_[\w.~+-]+_(?:windows-x64\.zip|amd64\.deb)', text):
        if asset not in assets:
            errors.append(f'{name}: stale package filename {asset}')

if errors:
    sys.exit('\n'.join(errors))
print(f'PASS: Cargo, web manifests and current text documentation agree on {version}')
