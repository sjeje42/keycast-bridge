# Keycast Bridge

[English](README.md) · **Français**

[![Rust stable](https://img.shields.io/badge/Rust-stable-000000?logo=rust&logoColor=white)](Cargo.toml)
[![GTK4](https://img.shields.io/badge/GTK-4-7FE719?logo=gtk&logoColor=white)](Cargo.toml)
[![Svelte](https://img.shields.io/badge/Svelte-5-FF3E00?logo=svelte&logoColor=white)](web/package.json)
[![Linux Debian 13](https://img.shields.io/badge/Linux-Debian_13-A81D33?logo=debian&logoColor=white)](docs/TESTING.md)
[![Wayland](https://img.shields.io/badge/Wayland-native-F0C674)](README.fr.md#fonctionnalités)
[![OBS Browser Source](https://img.shields.io/badge/OBS-Browser_Source-302E31?logo=obsstudio&logoColor=white)](README.fr.md#premier-tutoriel-dans-obs)
[![Build and test](https://github.com/sjeje42/keycast-bridge/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sjeje42/keycast-bridge/actions/workflows/ci.yml)
[![License GPL-3.0-only](https://img.shields.io/badge/License-GPL--3.0--only-blue)](LICENSE)

**Afficher les raccourcis clavier dans OBS, sous Linux et Wayland.**

Version **0.1.0-alpha.1**. Nouveau projet Rust / GTK4 / Svelte, sous GPL-3.0-only. Cible prioritaire : Debian 13, GNOME, OBS en paquet Debian. Le fonctionnement sur d’autres compositeurs reste à tester.

![Aperçu de l’overlay OBS](docs/overlay-preview.png)

[Confidentialité et sécurité](SECURITY.md) · [Procédure de test](docs/TESTING.md) · [État de validation](docs/VALIDATION.md)

## Fonctionnalités

- Interface native GTK4 en français et en anglais ; moteur de capture et serveur local en Rust ; overlay Svelte et TypeScript.
- Capture d’un clavier choisi, avec autorisation administrateur puis abandon des privilèges.
- Dispositions AZERTY France, QWERTY US / Royaume-Uni et QWERTZ Allemagne, interprétées avec libxkbcommon.
- Raccourcis clavier dans une source Navigateur OBS transparente ; taille, durée et thème clair/sombre réglables.
- Arrêt par bouton ou **Ctrl+Alt+F12**, aperçu et démonstration sans accès au clavier.
- Aucun historique des frappes ni télémétrie ; accès à l’overlay limité à la machine locale avec une URL aléatoire par lancement.

## Installation depuis les sources

```sh
sudo apt install build-essential pkg-config libgtk-4-dev libxkbcommon-dev libxkbcommon-tools xkb-data nodejs npm cargo rustc pkexec
```

Utiliser Rust stable récent (les dépendances verrouillées peuvent demander une version plus récente que celle de Debian). Node.js 22 conseillé. Depuis le dossier du projet, **avec ton utilisateur habituel** :

```sh
./scripts/build.sh
sudo ./scripts/install.sh
```

Ouvrir **Keycast Bridge** dans les applications GNOME. L’interface propose un choix Français / English. Le serveur et l’interface ne tournent jamais en administrateur. Seule l’ouverture du clavier choisi demande une autorisation ; les privilèges sont ensuite abandonnés.

## Premier tutoriel dans OBS

1. Choisir le périphérique clavier et **AZERTY — France**, ou la disposition utilisée dans GNOME. Les périphériques non clavier sont refusés lors du démarrage.
2. Cliquer sur **Copier l’URL OBS**. Dans OBS : **Sources → + → Navigateur**. Coller l’URL, largeur 1920, hauteur 1080, ou les dimensions de ta scène.
3. Cliquer sur **Tester le rendu** : un raccourci fictif s’affiche sans lire ton clavier. **Ouvrir l’aperçu** affiche le rendu dans ton navigateur.
4. Régler taille, durée et thème. Le fond de la page reste transparent.
5. Cliquer sur **Démarrer**, donner l’autorisation demandée et vérifier **Capture active**.
6. **Arrêter**, ou **Ctrl+Alt+F12**, met fin à la capture et efface l’affichage.

**L’URL change à chaque lancement** : il faut la remettre dans OBS. Aucun jeton permanent n’est enregistré. Ne pas partager l’URL pendant la capture.

Si **Navigateur** est absent des sources OBS, ton paquet OBS ne fournit pas cette fonction. Il faudra un paquet/build OBS qui l’inclut. Le projet ne fournit pas encore de fenêtre overlay de remplacement.

## Quelles touches sont affichées ?

Par défaut : combinaisons avec Ctrl, Alt gauche ou Super ; touches de fonction ; navigation, Entrée, Tabulation, Échap et effacement. Les lettres seules et la saisie AltGr ne sont pas diffusées. Les répétitions lors d’un appui long sont ignorées.

L’option **Afficher aussi les touches de texte** permet les raccourcis à une seule lettre de certains logiciels. Elle peut révéler du texte privé : à activer seulement pour les démonstrations qui le nécessitent. Le programme n’est pas un outil de transcription de phrases.

## Confidentialité et limites

- Capture arrêtée au lancement ; aucun historique des touches, télémétrie ni service distant.
- Serveur limité à `127.0.0.1`, URL aléatoire et vérification d’origine.
- **Aucune détection des champs de mot de passe.** Arrêter avant toute saisie sensible.
- **Pas d’arrêt automatique au verrouillage de GNOME.** Arrêter avant de verrouiller/changer de session.
- Un seul clavier à la fois. Après débranchement : actualiser, sélectionner à nouveau et redémarrer.
- Changement de disposition dans GNOME : arrêter, choisir la nouvelle disposition, redémarrer.
- Pas encore de souris, de composition des accents/Compose/IME, de préférences persistantes, de paquet `.deb` ou de synchronisation initiale Verr. Maj/Verr. Num.
- Relâcher les modificateurs avant de lancer la capture.

C’est une version alpha : les essais sur un véritable clavier sous GNOME et dans OBS restent indispensables avant une diffusion publique. Voir [la procédure de test](docs/TESTING.md).

## Démonstration sans clavier

Après installation, lancer `keycast-bridge-demo` dans un terminal et copier l’URL affichée dans OBS. Il diffuse uniquement quatre raccourcis fictifs. Quitter avec Ctrl+C. Ne pas lancer le mode démonstration et l’application simultanément : ils utilisent le même port.

## Désinstallation

```sh
sudo ./scripts/uninstall.sh
```

Le script retire uniquement les fichiers installés de Keycast Bridge.

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
