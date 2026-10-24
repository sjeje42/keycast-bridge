## Français — 0.2.0 stable

Première version stable après les retours positifs de tests prolongés par des utilisateurs faisant de la formation.

- **Paramètres → Anneau au clic** sous Windows : couleur indépendante, rayon de base de 10 à 200 px, épaisseur de 1 à 20 px (limitée au rayon), effet néon et pulsation activables séparément.
- Aperçu animé réduit, réglages appliqués en direct et mémorisés, bouton de réinitialisation de l’anneau.
- Dimensions exprimées dans le canevas OBS ; le cercle reste centré sur le clic, quel que soit le positionnement des touches. Sans pulsation, il conserve son rayon et disparaît en fondu après 0,5 s.
- Anciennes préférences conservées ; les nouveaux réglages reçoivent leurs valeurs par défaut.
- Guides en ligne et aide HTML français/anglais actualisés. Les PDF illustrés existants restent inclus dans leur édition précédente ; les nouveaux réglages sont décrits dans l’aide intégrée.

**Windows x64** : extraire entièrement `keycast-bridge_0.2.0_windows-x64.zip` et lancer `keycast-bridge.exe`.

**Debian 13 amd64** : `sudo apt install ./keycast-bridge_0.2.0-1_amd64.deb`.

L’anneau demeure une fonction Windows, visible au clic dans OBS. La version Linux conserve l’affichage des touches et des boutons de souris. Les exécutables Windows restent non signés ; la préparation SignPath n’est pas une signature active. Les empreintes des fichiers sont dans `SHA256SUMS`.

## English — 0.2.0 stable

First stable release following positive extended field-testing feedback from users who deliver training.

- Windows **Settings → Click ring**: independent color, 10–200px base radius, 1–20px thickness (capped by radius), independently selectable neon and pulse effects.
- Reduced animated preview, live saved settings and a ring-only reset button.
- Dimensions use OBS canvas pixels; the ring remains centered on the click independently of key placement. Without pulse it retains its radius and fades away after 0.5s.
- Existing preferences are retained; new settings receive defaults.
- Updated English/French online and embedded HTML guides. Existing illustrated PDFs are included in their earlier edition; new controls are documented in embedded help.

**Windows x64**: extract all of `keycast-bridge_0.2.0_windows-x64.zip` and launch `keycast-bridge.exe`.

**Debian 13 amd64**: `sudo apt install ./keycast-bridge_0.2.0-1_amd64.deb`.

The click ring remains Windows-only and appears in OBS on clicks. Linux retains key and mouse-button feedback. Windows executables remain unsigned; SignPath preparation is not active signing. File checksums are in `SHA256SUMS`.
