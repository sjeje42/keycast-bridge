# Keycast Bridge — documentation française

**Afficher les raccourcis clavier dans OBS, sous Linux et Wayland.**

Version **0.1.0-alpha.1**. Nouveau projet Rust / GTK4 / Svelte, sous GPL-3.0-only. Cible prioritaire : Debian 13, GNOME, OBS en paquet Debian. Le fonctionnement sur d’autres compositeurs reste à tester.

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

C’est une version alpha : les essais sur un véritable clavier sous GNOME et dans OBS restent indispensables avant une diffusion publique. Voir [la procédure de test](TESTING.md).

## Démonstration sans clavier

Après installation, lancer `keycast-bridge-demo` dans un terminal et copier l’URL affichée dans OBS. Il diffuse uniquement quatre raccourcis fictifs. Quitter avec Ctrl+C. Ne pas lancer le mode démonstration et l’application simultanément : ils utilisent le même port.

## Désinstallation

```sh
sudo ./scripts/uninstall.sh
```

Le script retire uniquement les fichiers installés de Keycast Bridge.

## Préparer le dépôt GitHub

Créer un dépôt vide nommé `keycast-bridge` sur GitHub, sans README/licence générés (ils sont déjà ici), puis depuis ce dossier :

```sh
git init -b main
git add .
git commit -m "Initial Keycast Bridge alpha"
git remote add origin git@github.com:sjeje42/keycast-bridge.git
git push -u origin main
```

Le workflow fourni compilera l’application et exécutera les tests lors du push. Ne publier une version stable qu’après les essais matériels.
