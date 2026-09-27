## Français — 0.2.0-alpha.4

**Touches maintenues : Maj, Ctrl, Alt et leurs combinaisons restent visibles pendant les clics et les glissers.**

- Une ligne en direct affiche les modificateurs dès l’appui et les retire au relâchement, sans expiration pendant le maintien.
- Le dernier raccourci garde son affichage temporaire sur une ligne distincte.
- Fonctionne sous Windows et Linux, avec ou sans affichage de la souris. Aucun besoin d’activer les touches de texte.
- AltGr reste identifié séparément. Les côtés gauche/droit et plusieurs claviers Linux sont pris en compte.
- Débranchement, arrêt et perte de connexion effacent les états obsolètes. Une reconnexion OBS récupère les modificateurs encore maintenus.
- Le choix multi-écran Windows de l’alpha.3 est conservé.

Pour Photoshop/Affinity : activer « Afficher les clics de souris », démarrer, maintenir Maj/Ctrl/Alt puis cliquer ou glisser. La touche apparaît avec un contour violet tant qu’elle reste enfoncée. Le cercle à la position du pointeur reste réservé à Windows.

Windows : extraire entièrement le ZIP et lancer `keycast-bridge.exe`.
Debian 13 : `sudo apt install ./keycast-bridge_0.2.0.alpha.4-1_amd64.deb`.

## English

Held Shift, Ctrl, Alt and combinations now remain visible through mouse clicks and drags. A live row follows press/release independently of the timed shortcut row. Available on Windows and Linux without enabling text capture. Includes left/right modifier handling, Linux keyboard aggregation, AltGr, disconnection cleanup and OBS reconnect state. Windows monitor selection remains available.

Automated tests cover modifier state, hotplug and overlay behavior; real Photoshop/Affinity workflows still need user validation.
