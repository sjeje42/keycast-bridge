# Keycast Bridge

Guides en ligne : [Français](docs/USER_GUIDE.fr.md) · [English](docs/USER_GUIDE.en.md)

Guides PDF : [Français](docs/pdf/Keycast_Bridge_Guide_Utilisateur_FR.pdf) · [English](docs/pdf/Keycast_Bridge_User_Guide_EN.pdf)

[English](README.md) · **Français**

[![Rust stable](https://img.shields.io/badge/Rust-stable-000000?logo=rust&logoColor=white)](Cargo.toml)
[![GTK4](https://img.shields.io/badge/GTK-4-7FE719?logo=gtk&logoColor=white)](Cargo.toml)
[![Svelte](https://img.shields.io/badge/Svelte-5-FF3E00?logo=svelte&logoColor=white)](web/package.json)
[![Linux Debian 13](https://img.shields.io/badge/Linux-Debian_13-A81D33?logo=debian&logoColor=white)](docs/TESTING.md)
[![Windows 10 / 11 portable x64](https://img.shields.io/badge/Windows-10%20%2F%2011%20portable%20x64-0078D4)](docs/WINDOWS.md)
[![Wayland](https://img.shields.io/badge/Wayland-native-F0C674)](README.fr.md#fonctionnalités)
[![OBS Browser Source](https://img.shields.io/badge/OBS-Browser_Source-302E31?logo=obsstudio&logoColor=white)](README.fr.md#premier-tutoriel-dans-obs)
[![Build and test](https://github.com/sjeje42/keycast-bridge/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sjeje42/keycast-bridge/actions/workflows/ci.yml)
[![License GPL-3.0-only](https://img.shields.io/badge/License-GPL--3.0--only-blue)](LICENSE)

**Afficher les raccourcis clavier dans OBS, sous Linux/Wayland et Windows.**

## Installation rapide

[Télécharger la version 0.2.0-alpha.8](https://github.com/sjeje42/keycast-bridge/releases/tag/v0.2.0-alpha.8)

[Code signing policy — politique de signature](CODE_SIGNING_POLICY.md) : candidature SignPath Foundation en préparation ; les versions Windows actuelles restent non signées.

- **Windows 10/11 x64 :** télécharger le ZIP, l’extraire entièrement et lancer `keycast-bridge.exe`.
- **Debian 13 amd64 :** télécharger le `.deb`, exécuter `sudo apt install ./keycast-bridge_0.2.0.alpha.8-1_amd64.deb`, puis ouvrir **Keycast Bridge** dans le menu des applications.

Windows : les exécutables ne sont pas signés ; SmartScreen peut afficher un avertissement. Voir les [informations antivirus et la vérification SHA256](docs/WINDOWS.md#smartscreen-antivirus-et-intégrité).

## Présentation

Version **0.2.0-alpha.8**. Nouveau projet Rust / GTK4 / Svelte, sous GPL-3.0-only. Essais manuels confirmés sous Debian 13 (GNOME), Fedora, Windows 10 et Windows 11. La version et le bureau utilisés sous Fedora ne sont pas consignés ; cela ne valide pas tous les compositeurs Wayland. Voir [le périmètre des validations](docs/VALIDATION.md).

![Aperçu de l’overlay OBS](docs/overlay-preview.png)

[Confidentialité et sécurité](SECURITY.md) · [Procédure de test](docs/TESTING.md) · [État de validation](docs/VALIDATION.md)

## Fonctionnalités

- Interface native GTK4 en français et en anglais ; moteur de capture et serveur local en Rust ; overlay Svelte et TypeScript.
- Linux : tous les claviers automatiquement ou sélection multiple, branchement/débranchement via udev sans redémarrer ; autorisation Polkit, ouverture privilégiée et lecteur sans privilèges.
- Windows : capture native de tous les claviers, sans administrateur, avec disposition de la fenêtre active.
- Clics gauche/droit/milieu optionnels, avec bouton coloré ; cercle au clic sous Windows, en mono-écran ou sur le moniteur choisi en multi-écran. Cocher **Cercle au clic** avant de démarrer active aussi la capture souris. L’unique écran est sélectionné automatiquement, même après débranchement d’un autre écran.
- Sous Linux : dispositions AZERTY France, QWERTY US / Royaume-Uni et QWERTZ Allemagne, interprétées avec libxkbcommon. Sous Windows : disposition de la fenêtre active.
- Raccourcis et modificateurs maintenus dans une source Navigateur OBS transparente ; position libre, couleurs personnalisables, taille/durée des touches et dimensions du format réglables. Position, couleurs et format sont mémorisés.
- Arrêt par bouton ou **Ctrl+Alt+F12**, aperçu et démonstration sans accès au clavier.
- Aucun historique des frappes ni télémétrie ; accès à l’overlay limité à la machine locale avec une URL aléatoire par lancement.

## Windows portable (x64)

Télécharger le ZIP depuis les [Releases](https://github.com/sjeje42/keycast-bridge/releases/tag/v0.2.0-alpha.8), l’extraire entièrement, puis lancer `keycast-bridge.exe`. Aucun outil de compilation à installer. [Installation, halo et limites Windows](docs/WINDOWS.md).

## Paquet Debian 13 (amd64)

Télécharger le `.deb` depuis les [Releases GitHub](https://github.com/sjeje42/keycast-bridge/releases), puis ouvrir un terminal dans le dossier du téléchargement :

```sh
sudo apt install ./keycast-bridge_0.2.0.alpha.8-1_amd64.deb
```

Lancer **Keycast Bridge** depuis le menu des applications. Aucune compilation nécessaire. APT installe les dépendances ; le paquet fournit le composant de capture et la règle Polkit. Aucune capture automatique. Désinstallation : `sudo apt remove keycast-bridge`.

Cible : **Debian 13, PC Intel/AMD 64 bits**. Ubuntu et les autres versions de Debian ne sont pas validés. Désinstaller une ancienne installation depuis les sources avec son script avant d’installer le paquet. Le composant seul installé manuellement dans `/usr/local/libexec` peut rester : le paquet utilise `/usr/libexec`.

**OBS :** le paquet Debian ne fournit pas la source Navigateur. Utiliser le [Flatpak officiel d’OBS](https://obsproject.com/kb/linux-installation), qui l’inclut. Keycast Bridge reste installé avec le paquet Debian.

## Plateformes et validation

Les essais manuels et les tests automatisés sont distingués dans [VALIDATION.md](docs/VALIDATION.md). Voir aussi [l’index de documentation](docs/README.md) et [l’historique des versions](CHANGELOG.md).

| Plateforme | Méthode | Validation |
| --- | --- | --- |
| Debian 13 | Paquet ci-dessus ou compilation | Capture/OBS confirmés sur GNOME ; installation du paquet testée en conteneur |
| Ubuntu 24.04 LTS et versions ultérieures | Compilation avec APT | Compilation et tests automatisés sur Ubuntu 24.04 ; capture sur poste réel à valider |
| Linux Mint 22.x (base Ubuntu 24.04) | Même procédure qu’Ubuntu | À valider sur poste réel |
| Fedora | Compilation avec DNF | Utilisation sur poste réel confirmée ; version et environnement de bureau non consignés |
| Manjaro / Arch Linux à jour | Compilation avec Pacman | Procédure proposée, pas encore testée sur ces distributions |
| Windows 10 x64 | ZIP portable | Utilisation sur poste réel confirmée ; numéro de build du système non consigné |
| Windows 11 x64 | ZIP portable | Utilisation sur poste réel confirmée ; numéro de build du système non consigné |

Le paquet `.deb` fourni reste destiné à Debian 13. Pour les autres distributions, suivre les étapes ci-dessous. Le fonctionnement sur tous les compositeurs Wayland n’est pas encore validé.

## Installation depuis les sources — autres distributions Linux

Cette méthode compile Keycast Bridge sur la machine cible, sans créer de paquet. **GTK 4.8 minimum**, Rust stable récent, Node.js 22 et npm sont nécessaires. Les commandes ci-dessous sont prévues pour Bash.

### 1. Installer les dépendances de sa distribution

**Debian 13 / Ubuntu 24.04+ / Linux Mint 22.x :**

```sh
sudo apt update
sudo apt install build-essential pkg-config git curl ca-certificates libgtk-4-dev libxkbcommon-dev libudev-dev xkb-data pkexec polkitd xdg-utils
```

**Fedora Workstation (installation classique avec DNF) :**

```sh
sudo dnf install gcc gcc-c++ make pkgconf-pkg-config git curl ca-certificates gtk4-devel libxkbcommon-devel systemd-devel xkeyboard-config polkit xdg-utils
```

Cette procédure ne couvre pas Fedora Silverblue/Kinoite et les autres variantes immuables.

**Manjaro / Arch Linux :**

```sh
sudo pacman -Syu --needed base-devel pkgconf git curl ca-certificates gtk4 libxkbcommon systemd xkeyboard-config polkit xdg-utils
```

Cette commande met également le système à jour pour éviter une mise à jour partielle. Redémarrer si la mise à jour du système le demande, puis reprendre ici.

### 2. Préparer Rust et Node.js

Installer [Rust avec rustup](https://rust-lang.org/tools/install/) avec son utilisateur habituel. Si rustup est déjà installé, passer directement à la commande de mise à jour :

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable
. "$HOME/.cargo/env"
rustup update stable
```

Pour utiliser **Node.js 22 avec npm**, comme dans les tests GitHub, installer [nvm](https://github.com/nvm-sh/nvm#installing-and-updating) si nécessaire, sans `sudo` :

```sh
curl -fsSL https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.8/install.sh | bash
export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "$HOME/.nvm" || printf %s "$XDG_CONFIG_HOME/nvm")"
. "$NVM_DIR/nvm.sh"
nvm install 22
nvm use 22
```

Si Node.js 22 et npm sont déjà disponibles, l’étape nvm est facultative. Vérifier les outils :

```sh
rustup run stable rustc --version
node --version
npm --version
pkg-config --modversion gtk4
command -v pkexec
```

### 3. Télécharger, compiler et installer

```sh
git clone https://github.com/sjeje42/keycast-bridge.git
cd keycast-bridge
RUSTUP_TOOLCHAIN=stable sh scripts/build.sh
sudo sh scripts/install.sh
```

Autre possibilité : télécharger **Code → Download ZIP**, extraire l’archive et ouvrir un terminal dans le dossier contenant `Cargo.toml`. Utiliser l’archive complète des sources, comprenant `scripts/` et `data/`.

La compilation s’effectue **sans sudo**. Le script d’installation place les exécutables dans `/usr/local/bin`, le composant de capture dans `/usr/local/libexec`, et installe le lanceur et la règle Polkit. Cette étape est indispensable : lancer seulement l’exécutable compilé n’installe pas le composant de capture.

Si le paquet Debian est déjà installé, le retirer avec `sudo apt remove keycast-bridge` avant une installation depuis les sources : les deux méthodes partagent la règle Polkit.

### 4. Lancer et configurer OBS

Ouvrir **Keycast Bridge** depuis le menu des applications, ou exécuter `/usr/local/bin/keycast-bridge`, sans sudo. Dans une session graphique minimale, un agent d’authentification Polkit doit être actif pour afficher la demande d’autorisation.

OBS doit proposer **Sources → + → Navigateur**. Si cette source manque, suivre les [instructions officielles OBS pour Linux](https://obsproject.com/kb/linux-installation) pour installer le Flatpak officiel. Keycast Bridge peut rester installé nativement sur la distribution. Voir la configuration détaillée ci-dessous.

### Mise à jour et désinstallation de la version compilée

Fermer Keycast Bridge avant de réinstaller. Depuis un clone Git sans modifications locales :

```sh
git pull --ff-only
RUSTUP_TOOLCHAIN=stable sh scripts/build.sh
sudo sh scripts/install.sh
```

Avec une archive ZIP, télécharger les nouvelles sources et refaire la compilation/installation. Pour désinstaller **la version compilée**, depuis le dossier des sources :

```sh
sudo sh scripts/uninstall.sh
```

Pour **le paquet Debian**, utiliser uniquement `sudo apt remove keycast-bridge`, pas le script.

Noms des dépendances et commandes de gestion des paquets : [Ubuntu](https://packages.ubuntu.com/noble/libgtk-4-dev), [Fedora GTK4](https://packages.fedoraproject.org/pkgs/gtk4/gtk4-devel/), [Fedora libxkbcommon](https://packages.fedoraproject.org/pkgs/libxkbcommon/libxkbcommon-devel/), [Arch GTK4](https://archlinux.org/packages/extra/x86_64/gtk4/), [Manjaro Pacman](https://wiki.manjaro.org/index.php/Pacman_Overview).

## Premier tutoriel dans OBS

1. Linux : conserver **Tous les claviers (automatique)** ou choisir **Claviers sélectionnés** et cocher les périphériques. Choisir la disposition utilisée sur le bureau. Windows : disposition automatique.
2. Cliquer sur **Copier l’URL OBS**. Dans OBS : **Sources → + → Navigateur**. Coller l’URL et reporter la largeur/hauteur choisies dans **Paramètres → Format et taille** (1920 × 1080 par défaut).
3. Cliquer sur **Tester le rendu** : un raccourci fictif s’affiche sans lire ton clavier. **Ouvrir l’aperçu** affiche le rendu dans ton navigateur.
4. Régler taille, durée et thème. Le fond de la page reste transparent.
5. Cliquer sur **Démarrer**, donner l’autorisation demandée sous Linux et vérifier **Capture active**.
6. **Arrêter**, ou **Ctrl+Alt+F12**, met fin à la capture et efface l’affichage.

**L’URL change à chaque lancement** : il faut la remettre dans OBS. Aucun jeton permanent n’est enregistré. Ne pas partager l’URL pendant la capture.

Si **Navigateur** est absent des sources OBS, ton paquet OBS ne fournit pas cette fonction. Il faudra un paquet/build OBS qui l’inclut. Le projet ne fournit pas encore de fenêtre overlay de remplacement.

## Quelles touches sont affichées ?

Par défaut : combinaisons avec Ctrl, Alt gauche ou Super ; touches de fonction ; navigation, Entrée, Tabulation, Échap et effacement. Les lettres seules et le texte saisi avec AltGr ne sont pas diffusés. Maj, Ctrl, Alt, Win/Super et AltGr restent visibles tant qu’ils sont maintenus, même sans autre touche. Les répétitions lors d’un appui long sont ignorées.

L’option **Afficher aussi les touches de texte** permet les raccourcis à une seule lettre de certains logiciels. Elle peut révéler du texte privé : à activer seulement pour les démonstrations qui le nécessitent. Le programme n’est pas un outil de transcription de phrases.

## Apparence et paramètres

### Touches maintenues et dessin à la souris

Maj (`Shift`), Ctrl, Alt et leurs combinaisons restent affichés dans une ligne en direct tant qu’ils sont enfoncés, y compris pendant un clic ou un glisser avec la Plume. Un contour de la couleur d’accent choisie distingue les touches maintenues du dernier raccourci, affiché temporairement au-dessus. Disponible sous Linux et Windows, sans activer les touches de texte. Activez « Afficher les clics de souris » pour voir aussi les boutons.

### Position et couleurs

Ouvrir la roue crantée en haut à droite → **Position et couleurs**, puis glisser le bloc dans l’aperçu schématique. Les neuf emplacements prédéfinis et les champs **X/Y (%)** permettent aussi de le positionner au clavier. 0 % correspond au bord gauche/haut et 100 % au bord droit/bas de l’espace disponible, avec une marge pour garder les touches visibles. Les deux lignes et l’icône souris se déplacent ensemble.

Choisir séparément le fond, les touches, le texte et les trois couleurs de clic. L’accent sert aussi au contour des modificateurs et au cercle au clic. Les boutons de palette claire/sombre conservent la position ; **Tout réinitialiser** remet position et couleurs par défaut.

Les changements s’appliquent immédiatement dans OBS, même pendant la capture. L’aperçu GTK est schématique : **Tester le rendu** permet de vérifier le résultat exact dans OBS, mais arrête la capture en cours. Les coordonnées sont relatives à la source Navigateur, pas à une fenêtre d’un autre logiciel ; attention au recadrage de cette source dans OBS. Le cercle de clic reste attaché au pointeur.

Position, couleurs et format sont enregistrés dans `%APPDATA%\keycast-bridge\appearance.json` sous Windows, et `$XDG_CONFIG_HOME/keycast-bridge/appearance.json` ou `~/.config/keycast-bridge/appearance.json` sous Linux. Ce fichier ne contient aucune frappe ni URL OBS. Les autres options restent limitées à la session. En cas d’échec de sauvegarde, l’interface le signale.

### Paramètres et format OBS

La roue crantée en haut à droite ouvre **Position et couleurs**, **Format et taille** et **Capture**. La fenêtre principale garde les commandes d’enregistrement ; les réglages sont dans une fenêtre séparée. Choisissez des dimensions standard ou personnalisées (160–7680 px par côté), puis reportez les mêmes valeurs dans les propriétés de la source Navigateur OBS. Le rendu s’ajuste sans déformation à une fenêtre de navigateur différente, avec des marges transparentes. Le format est mémorisé avec la position et les couleurs. **Guide complet** ouvre l’aide intégrée hors ligne ; les versions HTML/Markdown restent incluses dans les paquets. Sous Windows, le guide et l’aperçu utilisent le navigateur par défaut ; en cas d’échec, une boîte de dialogue propose **Copier le lien**. Garder Keycast Bridge ouvert pour consulter cette aide locale. Les PDF illustrés sont également téléchargeables séparément dans les Releases et dans `docs/pdf/` ; le ZIP Windows et le paquet Debian les incluent à côté des guides HTML.

## Confidentialité et limites

- Capture arrêtée au lancement ; aucun historique des touches, télémétrie ni service distant.
- Serveur limité à `127.0.0.1`, URL aléatoire et vérification d’origine.
- **Aucune détection des champs de mot de passe.** Arrêter avant toute saisie sensible.
- **Pas d’arrêt automatique au verrouillage de GNOME.** Arrêter avant de verrouiller/changer de session.
- Linux : les nouveaux claviers rejoignent automatiquement la capture en mode tous. En sélection, un clavier reconnu revient après rebranchement ; sans numéro de série, conserver le même port USB. Les modificateurs sont suivis séparément par clavier : effectuer une combinaison sur un même clavier.
- Les clics sont affichés dans l’incrustation sur les deux systèmes. Sous Wayland, aucun halo à la position du pointeur : evdev ne fournit pas les coordonnées globales du compositeur. Sous Windows, voir les contraintes d’alignement de l’écran sélectionné dans [WINDOWS.md](docs/WINDOWS.md).
- Changement de disposition dans GNOME : arrêter, choisir la nouvelle disposition, redémarrer.
- Pas encore de molette/déplacement du pointeur, de composition des accents/Compose/IME, de synchronisation initiale Verr. Maj/Verr. Num.
- La position, les couleurs et le format sont mémorisés. La taille, la durée, les options de capture, le moniteur choisi et la langue restent limités à la session.
- Relâcher les modificateurs avant de lancer la capture.

Les essais manuels sont confirmés sous Debian 13, Fedora, Windows 10 et Windows 11. Cette version reste une alpha : vérifier sa propre scène OBS avant une diffusion en direct. Les configurations non couvertes et les contrôles de régression sont décrits dans [VALIDATION.md](docs/VALIDATION.md) et [la procédure de test](docs/TESTING.md).

## Démonstration sans clavier

Après installation, lancer `keycast-bridge-demo` dans un terminal et copier l’URL affichée dans OBS. Il diffuse uniquement quatre raccourcis fictifs. Quitter avec Ctrl+C. Ne pas lancer le mode démonstration et l’application simultanément : ils utilisent le même port.

## Désinstallation

Paquet Debian : `sudo apt remove keycast-bridge`. Installation depuis les sources : `sudo sh scripts/uninstall.sh`, depuis le dossier des sources. Utiliser uniquement la méthode correspondant à son installation.

## Développement et tests

Depuis la racine du projet :

```sh
npm --prefix web ci
npm --prefix web run check
npm --prefix web run build
cargo test --locked --no-default-features
cargo run --locked --no-default-features --bin keycast-bridge-demo
```

Le frontend doit être compilé avant Rust : ses fichiers sont intégrés aux exécutables. Le workflow GitHub Actions contrôle le formatage et le code, exécute les tests unitaires et HTTP/WebSocket, puis compile les exécutables. Le badge en haut de page renvoie à son état actuel.

## Licence et remerciements

Copyright © 2026 Jérôme Stavrianos. Projet sous **GPL-3.0-only** : voir [LICENSE](LICENSE).

Keycast Bridge est une nouvelle implémentation inspirée du besoin couvert par Screenkey. Aucun code ni élément graphique de Screenkey n’est inclus. Les dépendances conservent leurs licences respectives.
