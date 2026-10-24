## Français — 0.2.0 stable

Première version stable après les retours positifs de tests prolongés par des utilisateurs faisant de la formation.

- **Paramètres → Anneau** : couleur indépendante, rayon de base de 10 à 200 px, épaisseur de 1 à 20 px (limitée au rayon), effet néon et pulsation activables séparément.
- **Présentation** : anneau du pointeur, six traînées, localisation en secouant la souris, spotlight cercle/carré avec rayon et assombrissement.
- Annotation et tableau blanc temporaires, couleur/épaisseur du trait, Ctrl+Z, Retour arrière et Échap. Zoom **figé** de 1,25× à 8×, déplacement et molette. Raccourcis Ctrl+Alt+D/B/Z sur l’écran du pointeur.
- Outils arrêtés au lancement ; nettoyage sur arrêt, verrouillage et perte de connexion. Extension GNOME 48 fournie dans le paquet Debian.
- Aperçu animé réduit, réglages appliqués en direct et mémorisés, bouton de réinitialisation de l’anneau.
- Dimensions exprimées dans le canevas OBS ; le cercle reste centré sur le clic, quel que soit le positionnement des touches. Sans pulsation, il conserve son rayon et disparaît en fondu après 0,5 s.
- Anciennes préférences conservées ; les nouveaux réglages reçoivent leurs valeurs par défaut.
- Guides en ligne et aide HTML français/anglais actualisés. Les PDF illustrés existants restent inclus dans leur édition précédente ; les nouveaux réglages sont décrits dans l’aide intégrée.

**Windows x64** : extraire entièrement `keycast-bridge_0.2.0_windows-x64.zip` et lancer `keycast-bridge.exe`.

**Debian 13 amd64** : `sudo apt install ./keycast-bridge_0.2.0-1_amd64.deb`.

Les outils du bureau fonctionnent sous Windows et Debian 13/GNOME 48 (extension GNOME incluse, à activer explicitement). Utiliser la capture d’écran OBS ; la source Navigateur seule affiche les touches et, sous Windows, le cercle au clic. Les exécutables Windows restent non signés ; la préparation SignPath n’est pas une signature active. Les empreintes des fichiers sont dans `SHA256SUMS`.

## English — 0.2.0 stable

First stable release following positive extended field-testing feedback from users who deliver training.

- **Settings → Ring**: independent color, 10–200px base radius, 1–20px thickness (capped by radius), independently selectable neon and pulse effects.
- **Presentation**: pointer ring, six trails, shake to locate, circle/square spotlight with radius and dimming.
- Temporary annotation and whiteboard, pen color/width, Ctrl+Z, Backspace and Escape. **Frozen** 1.25×–8× zoom, panning and wheel controls. Ctrl+Alt+D/B/Z shortcuts on the pointer’s monitor.
- Tools start off; teardown on stop, lock and connection loss. GNOME 48 extension bundled in the Debian package.
- Reduced animated preview, live saved settings and a ring-only reset button.
- Dimensions use OBS canvas pixels; the ring remains centered on the click independently of key placement. Without pulse it retains its radius and fades away after 0.5s.
- Existing preferences are retained; new settings receive defaults.
- Updated English/French online and embedded HTML guides. Existing illustrated PDFs are included in their earlier edition; new controls are documented in embedded help.

**Windows x64**: extract all of `keycast-bridge_0.2.0_windows-x64.zip` and launch `keycast-bridge.exe`.

**Debian 13 amd64**: `sudo apt install ./keycast-bridge_0.2.0-1_amd64.deb`.

Desktop tools support Windows and Debian 13/GNOME 48 (included GNOME extension, explicitly enabled). Use OBS Display Capture; the Browser Source alone displays keys and the Windows click ring. Windows executables remain unsigned; SignPath preparation is not active signing. File checksums are in `SHA256SUMS`.
