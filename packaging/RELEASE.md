## Français — 0.2.0-alpha.5

**Position libre et couleurs personnalisées pour l’incrustation.**

Dans « Position et couleurs » :
- Glisser le bloc dans l’aperçu schématique ou choisir un des neuf emplacements prédéfinis.
- Ajuster X et Y entre 0 et 100 % de l’espace disponible. Les raccourcis et les modificateurs/souris se déplacent ensemble, avec une marge de sécurité aux bords.
- Choisir les couleurs du fond, des touches, du texte, de l’accent/clic gauche, du clic droit et du clic molette. L’accent colore également le cercle au clic.
- Revenir à une palette claire/sombre ou réinitialiser la position et les couleurs.

Les changements s’appliquent en direct dans OBS, sans arrêter la capture. Le cercle reste à la position du pointeur. Les positions sont relatives à la source Navigateur OBS : une source recadrée dans OBS peut masquer une partie de l’incrustation.

Position et couleurs sont mémorisées pour les prochains lancements. L’aperçu GTK est schématique ; utiliser « Tester le rendu » pour vérifier la taille exacte dans OBS (ce bouton arrête la capture en cours, comme auparavant).

Windows : extraire entièrement le ZIP et lancer `keycast-bridge.exe`.
Debian 13 : `sudo apt install ./keycast-bridge_0.2.0.alpha.5-1_amd64.deb`.

## English

Free overlay placement with a draggable schematic preview, nine presets and X/Y percentage controls. Customize the background, key background, text, accent/left click, right click and middle click. The click ring uses the accent color and stays at the pointer independently of keyboard placement.

Changes apply live without stopping capture. Position and colors persist across launches. Light/dark palettes and reset are available. Coordinates are relative to the OBS Browser Source; cropping in OBS can hide content. “Test overlay” shows the exact output and stops active capture, as before.

Automated validation covers settings persistence/validation, live configuration, overlay geometry and colors, Linux/Windows builds and package launch. Physical OBS validation remains useful before a public release.
