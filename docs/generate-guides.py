"""Regenerate embedded/offline HTML: pip install Markdown==3.8.2; python docs/generate-guides.py."""
from pathlib import Path
import markdown

root = Path(__file__).resolve().parent
css = """
:root { color-scheme:light dark; font:17px/1.65 system-ui,sans-serif; }
body { max-width:960px; margin:auto; padding:28px 24px 80px; background:#f8fafc; color:#192638; }
nav { display:flex; gap:20px; border-bottom:2px solid #ccd5e2; padding-bottom:12px; }
a { color:#275bc3; } h1 { font-size:2.1rem; line-height:1.2; } h2 { margin-top:2.4rem; border-top:1px solid #ccd5e2; padding-top:1rem; } h3 { margin-top:1.5rem; }
code { font-size:.9em; background:#e8edf4; border-radius:4px; padding:2px 5px; overflow-wrap:anywhere; }
pre { background:#e8edf4; padding:16px; border-radius:8px; overflow:auto; } pre code { padding:0; }
table { display:block; overflow:auto; border-collapse:collapse; width:100%; font-size:.93rem; }
th,td { text-align:left; vertical-align:top; border:1px solid #cbd5e1; padding:10px 12px; } th { background:#e8edf4; }
@media(prefers-color-scheme:dark) { body { background:#111923; color:#e3eaf4; } a { color:#97b8ff; } code,pre,th { background:#213044; } th,td,h2,nav { border-color:#40536e; } }
@media print { body { color:#000; background:#fff; font-size:11pt; } nav { display:none; } h2,h3 { break-after:avoid; } tr { break-inside:avoid; } }
"""
for lang in ['en', 'fr']:
    source = (root / f'USER_GUIDE.{lang}.md').read_text(encoding='utf-8')
    # The first paragraph's Markdown language link targets the other HTML guide offline too.
    source = source.replace('USER_GUIDE.en.md', 'en.html').replace('USER_GUIDE.fr.md', 'fr.html')
    body = markdown.markdown(source, extensions=['tables', 'fenced_code', 'toc'])
    title = 'User guide' if lang == 'en' else 'Guide d’utilisation'
    html = f'<!doctype html><html lang="{lang}"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Keycast Bridge — {title}</title><style>{css}</style></head><body><nav><strong>Keycast Bridge</strong><a href="en.html">English</a><a href="fr.html">Français</a></nav><main>{body}</main></body></html>\n'
    (root / 'guide').mkdir(exist_ok=True)
    (root / 'guide' / f'{lang}.html').write_text(html, encoding='utf-8')
