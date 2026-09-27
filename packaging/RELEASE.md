## Français — 0.2.0-alpha.6

**Fenêtre principale compacte, Paramètres et format OBS configurable.**

- Roue crantée en haut à droite : trois onglets « Position et couleurs », « Format et taille » et « Capture ».
- Les commandes d’enregistrement restent dans la fenêtre principale, dimensionnée selon les écrans disponibles. Les Paramètres s’ouvrent séparément et se ferment sans arrêter la capture.
- Formats HD, Full HD, QHD, UHD/4K, vertical et carré, ou dimensions personnalisées de 160 à 7680 pixels par côté.
- Format mémorisé, aperçu proportionnel et rappel dynamique des dimensions près de l’URL OBS. La source Navigateur doit être réglée aux mêmes dimensions dans OBS ; l’application ne modifie pas ces propriétés à distance.
- Le rendu s’ajuste uniformément à une fenêtre de navigateur différente, avec des marges transparentes si les proportions diffèrent.
- Guides complets en anglais puis en français : accessibles hors ligne depuis « Guide complet » dans les Paramètres, et fournis dans les paquets en HTML et Markdown.

Les réglages de position/couleurs et de format restent applicables pendant la capture. « Tester le rendu » arrête la capture, comme auparavant. La taille, la durée et les options de capture restent limitées à la session.

Windows : extraire tout le ZIP et ouvrir `keycast-bridge.exe`.
Debian 13 : `sudo apt install ./keycast-bridge_0.2.0.alpha.6-1_amd64.deb`.

## English

Compact main window with a top-right Settings gear. Separate tabs organize position/colors, canvas/size and capture. The main window adapts its initial dimensions to detected displays; very small screens retain a scrolling fallback.

Choose a standard or custom OBS canvas (160–7680 pixels per side). Dimensions persist, the schematic preview follows the aspect ratio, and the main URL hint shows current values. Enter the same width/height manually in OBS Browser Source properties. A mismatched browser viewport is fitted uniformly with transparent margins.

Complete English and French guides are embedded for offline access through Settings and included as HTML/Markdown in both packages. Appearance/canvas changes stay live; Test overlay still stops capture. Size, duration and capture options remain session-only.
