## Français — 0.2.0-alpha.3

**Multi-écran Windows : choisir l’écran utilisé dans OBS pour le cercle au clic.**

- Nouveau sélecteur d’écran : DISPLAY1, DISPLAY2, DISPLAY3… avec résolution, position et indication du principal.
- Coordonnées calculées par rapport à l’écran choisi, même à gauche ou au-dessus du principal, en portrait ou avec une résolution différente.
- Liste actualisée automatiquement, changement d’écran possible pendant la capture et géométrie relue à chaque clic.
- Si l’écran sélectionné est débranché, le cercle est suspendu sans basculer sur un autre écran. Les raccourcis et les boutons de souris restent actifs.

Windows : extraire entièrement le ZIP, lancer `keycast-bridge.exe`, activer les clics et le cercle, puis choisir l’écran capturé. Dans OBS, aligner la capture de cet écran et la source Navigateur sur le même rectangle, avec les mêmes proportions. Les captures de fenêtres ou les recadrages ne sont pas alignés automatiquement. Le choix d’écran n’est pas enregistré après fermeture.

Debian 13 amd64 : `sudo apt install ./keycast-bridge_0.2.0.alpha.3-1_amd64.deb`. Sous Linux/Wayland, les touches et les boutons fonctionnent quel que soit l’écran ; le halo à la position du pointeur reste indisponible.

La publication exige la réussite des builds Linux, Debian et Windows. Les tests vérifient les calculs multi-écran et l’énumération Windows ; les configurations physiques avec plusieurs écrans et DPI différents restent à valider dans OBS.

## English

**Windows multi-monitor click ring:** choose the monitor captured by OBS using the new display selector. Resolution, desktop position and primary status are shown. Negative coordinates and portrait layouts are supported. The list refreshes automatically; selection can change during capture and geometry is read on each click.

Disconnecting the selected display suspends its ring without silently switching to another display. Keyboard and mouse-button feedback continue. Align the selected monitor capture and Browser Source to the same rectangle and aspect ratio. Window captures and cropping are not mapped automatically. Selection is session-only. Linux/Wayland retains keyboard/button feedback across screens without a pointer-position halo.

All three build pipelines must pass before publication. Geometry and native enumeration tests do not replace physical multi-monitor/DPI testing in OBS.
