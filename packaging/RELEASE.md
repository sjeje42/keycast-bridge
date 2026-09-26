## Français

Paquet prêt à installer pour **Debian 13, amd64 (PC Intel/AMD 64 bits)**.

Télécharger le `.deb`, puis dans son dossier :

```sh
sudo apt install ./keycast-bridge_0.1.0.alpha.2-1_amd64.deb
```

Lancer **Keycast Bridge** depuis le menu des applications. Interface FR/EN, capture avec autorisation administrateur, overlay transparent OBS. Aucune compilation nécessaire.

**OBS : utiliser une version avec source Navigateur, notamment le Flatpak officiel. Le paquet OBS de Debian ne fournit pas cette source.**

Cette alpha intègre l’installateur Debian, la règle Polkit et les dépendances. Construction sous Debian 13, tests Rust, installation/désinstallation/réinstallation et lancement GTK4 en écran virtuel automatisés. Le fonctionnement clavier et OBS a été confirmé par l’utilisateur sur la précédente alpha ; le nouveau paquet reste à valider sur un poste GNOME réel.

Une ancienne installation complète depuis les sources dans `/usr/local` doit être désinstallée avec son script avant ce paquet. Le composant seul installé manuellement dans `/usr/local/libexec` peut rester : le paquet utilise `/usr/libexec`.

## English

Ready-to-install **Debian 13 amd64** package. Download the `.deb`, then run the command above in its directory. Launch **Keycast Bridge** from your applications menu. Dependencies, capture helper and Polkit policy are included or installed by APT. No compilation needed.

OBS requires **Browser Source**; use the official OBS Flatpak. Debian's OBS package does not include it.

Built and tested on Debian 13, including Rust tests and a clean-container install, virtual-display GTK4 launch, removal and reinstall. Real keyboard/OBS operation was confirmed by the user on the previous alpha; the new package still needs testing on a real GNOME desktop. Ubuntu and other Debian versions are not validated.

Uninstall an old complete source installation in `/usr/local` first. A standalone manually installed helper may remain; this package uses `/usr/libexec`.
