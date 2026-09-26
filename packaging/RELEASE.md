## Français — 0.2.0-alpha.2

- **Windows x64 portable** : extraire entièrement le ZIP puis lancer `keycast-bridge.exe`. GTK et ses DLL sont inclus ; aucun outil de compilation ni droit administrateur nécessaire.
- **Linux : branchement/débranchement à chaud** via udev, tous les claviers par défaut ou sélection multiple. Un processus privilégié ouvre les périphériques ; le lecteur travaille sans privilèges.
- **Clics de souris optionnels** : boutons gauche, droit et milieu mis en couleur dans l’incrustation.
- **Cercle au clic Windows** : écran principal complet, avec les deux sources OBS alignées. Sous Linux/Wayland, seule la visualisation des boutons est fournie.

Debian 13 amd64 : `sudo apt install ./keycast-bridge_0.2.0.alpha.2-1_amd64.deb`.

OBS doit fournir la source **Navigateur**. Sur Debian, utiliser le Flatpak officiel OBS. L’URL change à chaque lancement. Relâcher les modificateurs avant de démarrer ; arrêter avant de saisir des informations sensibles. Aucune détection des mots de passe.

La publication est conditionnée aux tests Linux, à la construction/installation du paquet Debian et aux tests de l’archive Windows. Les nouveaux comportements USB et souris, le DPI et l’alignement OBS restent à vérifier sur du matériel réel. [Guide Windows](https://github.com/sjeje42/keycast-bridge/blob/main/docs/WINDOWS.md) · [Tests manuels](https://github.com/sjeje42/keycast-bridge/blob/main/docs/TESTING.md).

## English

Portable Windows x64 build, Linux udev hotplug with automatic or multiple keyboard selection, optional colored mouse-button feedback and a Windows primary-monitor click ring. Extract the entire Windows ZIP; run as a normal user. Debian 13 amd64 installation command above.

Release publication requires passing Linux CI, Debian package install/launch tests and Windows portable tests. Physical USB/mouse capture, DPI and OBS alignment still require desktop validation. Wayland has button feedback only, no pointer-position ring. No password detection; stop before sensitive input. Overlay URL changes every launch.
