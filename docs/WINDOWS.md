# Windows — 0.2.0-alpha.6

## Français

Cible : Windows 10 1703 ou plus récent / Windows 11, Intel/AMD 64 bits.

1. Télécharger l’archive `keycast-bridge_0.2.0-alpha.6_windows-x64.zip` depuis les Releases ou les artifacts du workflow **Windows portable**.
2. Extraire **tout** le dossier. Conserver les DLL, `lib/`, `share/` et `licenses/` avec les exécutables.
3. Lancer `keycast-bridge.exe` avec son utilisateur habituel, sans administrateur. Aucun Rust, GTK ou MSYS2 à installer.
4. Choisir Français si nécessaire, copier l’URL OBS et créer une source **Navigateur** transparente aux dimensions de la scène.
5. Activer éventuellement **Afficher les clics de souris**, puis **Démarrer**. **Ctrl+Alt+F12** ou **Arrêter** termine la capture.

Tous les claviers de la session sont capturés. La disposition suit la fenêtre active. La sélection individuelle des périphériques est réservée à Linux dans cette alpha.

Le **cercle au clic** est optionnel : il correspond à l’**écran sélectionné** dans « Écran capturé dans OBS ». Choisir DISPLAY2, DISPLAY3, etc. selon le moniteur utilisé ; sa résolution et sa position sont indiquées. Ces identifiants Windows ne correspondent pas nécessairement à l’ordre des sources dans OBS. Dans OBS, la capture de cet écran et la source Navigateur doivent couvrir exactement le même rectangle, sans recadrage ni décalage. Les clics sur les autres écrans montrent le bouton mais pas de cercle. La liste est actualisée automatiquement. Le choix peut être changé pendant la capture. Si l’écran choisi disparaît, le cercle est suspendu sans basculer sur un autre écran ; choisir un écran connecté ou rebrancher celui-ci. La géométrie est relue à chaque clic, notamment après un changement de résolution. Le choix est conservé pendant cette session, pas après fermeture de l’application.

Le halo est dessiné dans OBS, pas sur le bureau Windows. Les captures de fenêtres, jeux ou régions recadrées ne sont pas alignées automatiquement.

L’archive n’est pas signée. Aucun historique ni télémétrie. Les champs de mot de passe ne sont pas détectés ; arrêter avant toute saisie sensible. Les bureaux sécurisés/UAC et les applications élevées ne font pas partie de la cible. Relâcher les modificateurs avant de démarrer. Pas de composition IME/accents. La position, les couleurs et le format de l’incrustation sont mémorisés ; les autres options restent limitées à la session. L’URL change à chaque lancement.

Les tests automatiques vérifient la compilation, les traductions clavier, le serveur et le démarrage de l’archive. La capture réelle, le branchement USB, le DPI et l’alignement OBS restent à vérifier sur un poste Windows. Voir `TESTING.md` dans les sources.

## English

Target: Windows 10 1703+ / Windows 11, x64 Intel/AMD. Extract the complete ZIP and launch `keycast-bridge.exe` as a normal user. Keep all DLLs and data directories alongside it. No development tools or separate GTK installation are needed. Copy the URL into an OBS Browser Source, enable mouse buttons if desired and press Start. Stop with the button or Ctrl+Alt+F12.

All session keyboards are captured; layout follows the foreground window. Per-device selection is Linux-only. The optional click ring maps the monitor selected under “Monitor captured in OBS” to the full browser viewport: align both OBS sources without cropping. Choose DISPLAY2, DISPLAY3, etc. using the resolution and position shown; these IDs need not match the order of OBS sources. The list refreshes automatically and selection can change during capture. If the selected display disappears, the ring is suspended without switching monitors. Reconnect it or select another monitor. Geometry refreshes on each click. The selection lasts for the current application session. Other monitors show button feedback only. The ring appears in OBS, not on the Windows desktop.

Unsigned alpha; no history or telemetry, no password detection. Stop before sensitive input or locking. Secure/elevated desktops are outside the target. Physical input, USB changes, DPI and OBS alignment still require desktop testing. The URL changes every launch.

## Building / Compilation (MSYS2 UCRT64)

Install Node.js 22, then run `npm --prefix web ci` and `npm --prefix web run build` from the project root. In MSYS2 UCRT64:

```sh
pacman -S --needed mingw-w64-ucrt-x86_64-rust mingw-w64-ucrt-x86_64-gcc mingw-w64-ucrt-x86_64-gtk4 mingw-w64-ucrt-x86_64-pkgconf python
cargo test --locked --release
cargo build --locked --release
/usr/bin/python packaging/windows/bundle.py
```

The portable directory is `dist/keycast-bridge-windows-x64`. See `.github/workflows/windows.yml` for the reproducible build steps and clean-PATH launch check. Bundled libraries retain their licenses, listed in `licenses/`; package versions and upstream source locations are recorded in `MSYS2-packages.txt`. MSYS2 package build recipes: https://github.com/msys2/MINGW-packages .


## Position et couleurs / Position and colors

Le panneau « Position et couleurs » propose un aperçu schématique à glisser, neuf emplacements, des coordonnées X/Y et six couleurs personnalisables. Les changements sont immédiats dans OBS. Position et couleurs sont enregistrées dans `%APPDATA%\keycast-bridge\appearance.json`. Le cercle reste à la position du pointeur.

The “Position and colors” panel provides a draggable schematic preview, nine presets, X/Y coordinates and six customizable colors. Changes apply live in OBS. Position and colors are saved in `%APPDATA%\keycast-bridge\appearance.json`. The click ring stays at the pointer.

## Guide et paramètres / Guide and settings

Ouvrir la roue crantée pour les onglets Position et couleurs, Format et taille, Capture. Le bouton Guide complet ouvre la documentation hors ligne en français ou anglais. Le dossier `guide` du ZIP contient également `en.html` et `fr.html`. Régler les propriétés de la source Navigateur OBS aux dimensions choisies dans l’application.

Open the Settings gear to configure appearance, canvas size or capture. Complete guide opens the offline help in the selected language. The ZIP also includes `guide/en.html` and `guide/fr.html`. Match OBS Browser Source width/height to the chosen canvas.
