# Application icon

The official artwork is kept unchanged in `keycast-bridge-2048.png` (2048 ×
2048 RGBA) and the supplied `keycast-bridge-original.ico` (256 × 256).

`hicolor/` contains PNG exports at 16, 24, 32, 48, 64, 128, 256, 512 and
1024 pixels. These are installed with the Linux desktop entry and embedded
as GTK resources so the application also has its icon when run from a build
directory. The Windows portable bundle includes the same theme files.

`keycast-bridge.ico` contains 16, 24, 32, 48, 64, 128 and 256 pixel images,
resampled from the original PNG with Pillow's Lanczos filter. Windows uses
this multi-resolution file for the executable and optional user shortcuts.
The 256px entry is PNG compressed; larger artwork remains available as PNG.

`build.rs` compiles the GTK resources with `glib-compile-resources` and, on
Windows UCRT64, the executable icon and version metadata with `windres`.
Both tools are supplied by the existing GTK/MSYS2 development dependencies.
Only the GUI executable receives the Windows resources. Resource embedding
does not sign the executable.
