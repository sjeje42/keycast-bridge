# Keycast Bridge — Guide d’utilisation

Version **0.2.0-alpha.8** · [English](USER_GUIDE.en.md)

Keycast Bridge affiche les raccourcis clavier, les modificateurs maintenus et, en option, les clics de souris dans une source Navigateur OBS transparente. L’application contrôle la capture et l’apparence ; OBS superpose le résultat à la capture de votre écran ou logiciel.

## 1. Installer et lancer

### Windows x64

Téléchargez le ZIP Windows dans les Releases GitHub du projet. Extrayez **toute l’archive** dans un dossier accessible en écriture, puis ouvrez `keycast-bridge.exe`. Conservez les DLL et les sous-dossiers à côté de l’exécutable. Il n’y a ni installateur, ni pilote, ni service permanent. Lancez l’application en utilisateur normal. Cette alpha n’est pas signée ; les bureaux sécurisés, les fenêtres UAC et les logiciels exécutés en administrateur ne font pas partie de la cible de capture prise en charge.

### Debian 13 amd64

Téléchargez le paquet Debian depuis les Releases, ouvrez un terminal dans son dossier et lancez :

```sh
sudo apt install ./keycast-bridge_0.2.0.alpha.8-1_amd64.deb
```

Ouvrez Keycast Bridge depuis le menu des applications. **Ne lancez pas l’interface avec sudo.** Le démarrage de la capture demande une autorisation administrateur via Polkit ; le composant de capture gère ensuite les périphériques. Il n’est pas nécessaire de vous ajouter au groupe `input`.

OBS doit proposer une **source Navigateur**. Si votre installation Linux ne l’inclut pas, le Flatpak officiel d’OBS est l’option documentée pour ce projet. Le paquet Debian de Keycast Bridge n’installe pas OBS.

### Autres distributions Linux

Les procédures de compilation pour Ubuntu 24.04+, Linux Mint 22.x, Fedora, Manjaro et Arch figurent dans le README du dépôt. Utilisez les dépendances et commandes correspondant à votre distribution ; le paquet Debian téléchargeable cible Debian 13. Les autres bureaux/compositeurs restent à valider sur poste réel. Les prérequis comprennent GTK 4.8+, Rust, Node.js 22 et les fichiers de développement de libudev et libxkbcommon.

### Mise à jour et désinstallation

Sous Windows, arrêtez la capture, fermez l’ancienne application et extrayez le nouveau ZIP dans un nouveau dossier. Sous Debian, installez le nouveau `.deb` avec APT. Les préférences d’apparence et de format sont conservées séparément et survivent aux mises à jour. Pour supprimer le paquet Debian, utilisez `sudo apt remove keycast-bridge` ; le fichier de préférences de l’utilisateur reste présent. Une seule instance peut utiliser le port du serveur local à la fois.

## 2. Première configuration dans OBS

1. Ouvrez Keycast Bridge. Changez la langue en haut de la fenêtre si nécessaire.
2. Cliquez sur la **roue crantée en haut à droite**, puis ouvrez **Format et taille**. Choisissez le format de la source, par exemple **1920 × 1080**.
3. Revenez à la fenêtre principale et cliquez sur **Copier l’URL OBS**.
4. Dans OBS, choisissez **Sources → + → Navigateur**. Créez une source et collez l’URL.
5. Renseignez la **même largeur et la même hauteur** dans les propriétés de la source Navigateur. Keycast Bridge rappelle les valeurs choisies au-dessus de l’URL.
6. Placez cette source **au-dessus** de la capture d’écran ou de logiciel dans la liste des sources OBS. Alignez leurs rectangles. Conservez la transparence du fond de l’incrustation.
7. Cliquez sur **Tester le rendu** dans Keycast Bridge. Un raccourci de démonstration apparaît brièvement. **Ce bouton arrête une capture en cours** : utilisez-le pour vérifier le résultat avant l’enregistrement.
8. Choisissez les options de capture, cliquez sur **Démarrer** et autorisez la capture sous Linux. Utilisez un raccourci ou maintenez Maj pour vérifier l’affichage en direct.

L’URL change à chaque redémarrage de Keycast Bridge. **Recopiez la nouvelle URL dans OBS après chaque lancement.** Une ancienne URL ne se reconnecte pas à la nouvelle session. Ne partagez pas cette adresse locale privée.

## 3. Fenêtre principale

La fenêtre principale compacte contient l’état de la capture, les options de souris, les boutons Démarrer/Arrêter/Tester, l’URL OBS, le format choisi et les commandes d’aperçu. Sa taille est adaptée aux écrans détectés. Une barre de défilement reste disponible sur les très petits écrans ou avec un fort agrandissement de l’interface.

| Commande | Utilité |
| --- | --- |
| Démarrer | Lancer la capture avec les options choisies. |
| Arrêter | Terminer la capture et effacer l’incrustation. |
| Ctrl + Alt + F12 | Arrêt d’urgence. La combinaison est également reçue par le logiciel actif. |
| Tester le rendu | Arrêter la capture et afficher un exemple pour vérifier OBS. |
| Copier l’URL OBS | Copier l’adresse locale de la session. |
| Ouvrir l’aperçu | Ouvrir l’incrustation dans le navigateur ; elle est transparente et peut sembler vide en l’absence de saisie affichée. |
| Roue crantée | Ouvrir les Paramètres sans interrompre la capture. |

Fermer les Paramètres laisse l’application et la capture actives. Fermer la fenêtre principale arrête la capture. L’apparence et le format peuvent changer pendant l’enregistrement. Pour changer les claviers, la disposition, l’affichage du texte ou la capture de souris, arrêtez puis redémarrez la capture.

## 4. Paramètres — Position et couleurs

### Placer l’incrustation

Glissez le bloc d’exemple dans l’aperçu schématique. Vous pouvez aussi choisir l’un des **neuf emplacements prédéfinis** ou saisir les **pourcentages X/Y** au clavier.

- X = 0 correspond au bord gauche ; X = 100 au bord droit.
- Y = 0 correspond au bord haut ; Y = 100 au bord bas.
- 50/50 centre le bloc. La position par défaut est 50/100, en bas au centre.
- Les pourcentages représentent **l’espace de déplacement disponible**, en tenant compte de la taille du bloc. Une marge garde les touches à distance des bords du format choisi.

La ligne du dernier raccourci et la ligne des modificateurs/souris se déplacent ensemble. Leur alignement suit la position horizontale. L’aperçu respecte les proportions du format choisi, mais les touches dessinées sont schématiques : vérifiez la taille réelle dans OBS. Un recadrage de la source Navigateur dans OBS peut toujours masquer du contenu.

### Personnaliser les couleurs

| Couleur | Éléments concernés |
| --- | --- |
| Fond | Le fond translucide de chaque ligne. |
| Touches | La surface de chaque touche. |
| Texte | Les lettres, les séparateurs et le contour de la souris. |
| Accent / clic gauche | Le contour des touches maintenues, le bouton gauche et le cercle au clic sous Windows. |
| Clic droit | La mise en évidence du bouton droit. |
| Clic molette | La mise en évidence du bouton central. |

Cliquez sur une pastille pour ouvrir le sélecteur de couleur. **Palette sombre** et **Palette claire** rétablissent un ensemble de couleurs en conservant la position et les dimensions. **Tout réinitialiser**, dans cet onglet, réinitialise seulement la position et les couleurs : le format et les options de capture restent inchangés.

## 5. Paramètres — Format et taille

**Taille** règle le texte des touches entre 20 et 96 pixels du format logique. **Durée** règle l’affichage du dernier raccourci entre 300 et 5000 ms. Les modificateurs maintenus restent visibles jusqu’au relâchement, quelle que soit cette durée.

Les formats prédéfinis sont 1280 × 720, 1920 × 1080, 2560 × 1440, 3840 × 2160, 1080 × 1920 et 1080 × 1080. Une largeur et une hauteur personnalisées peuvent chacune aller de 160 à 7680 pixels.

Ces valeurs définissent le **format logique de l’incrustation** et modifient son aperçu et son rendu. **Elles ne changent pas à distance les propriétés de la source Navigateur d’OBS.** Reportez les mêmes dimensions dans OBS pour obtenir le rendu prévu. Si la fenêtre du navigateur a d’autres dimensions, le format est réduit ou agrandi uniformément pour y tenir ; une différence de proportions laisse des marges transparentes, sans déformation. La taille apparente des touches peut donc varier si les dimensions ne correspondent pas.

Exemple : pour un tutoriel vertical en 1080 × 1920, choisissez ce format dans l’application, réglez la source Navigateur OBS sur 1080 × 1920 et alignez-la avec votre scène verticale. Pour le cercle au clic Windows sur une capture de moniteur entier, choisissez un format ayant les proportions de cet écran et alignez les deux sources sans recadrage.

## 6. Paramètres — Capture

### Claviers et disposition

Sous **Linux**, le réglage par défaut capture tous les claviers reconnus, y compris ceux branchés ensuite. Vous pouvez sélectionner des claviers précis dans les Paramètres et actualiser la liste. Choisissez une disposition correspondant à votre bureau : AZERTY France, QWERTY États-Unis ou Royaume-Uni, QWERTZ Allemagne. Débrancher un clavier efface son état de touches maintenues. En mode automatique, rebrancher ou remplacer un clavier ne demande pas de redémarrer OBS.

Sous **Windows**, tous les claviers de la session sont capturés et la disposition suit la fenêtre active. Cette alpha ne propose pas de sélection individuelle des claviers Windows.

### Raccourcis, texte et modificateurs

Par défaut, les caractères de texte ordinaires sont filtrés. Des raccourcis tels que Ctrl+C et des touches de contrôle nommées peuvent néanmoins apparaître. **Maj (`Shift`), Ctrl, Alt, Win/Super et AltGr** sont affichés dans une ligne dédiée tant qu’ils sont maintenus, même sans autre touche du clavier. Les variantes gauche/droite sont regroupées : relâcher un côté ne masque pas le modificateur encore maintenu de l’autre côté.

Cela permet notamment d’utiliser la Plume de Photoshop/Affinity : maintenez Maj, Ctrl ou Alt, puis cliquez ou glissez. Activez **Afficher les clics de souris** pour voir les boutons à côté des modificateurs. Le dernier raccourci reste dans une ligne temporaire distincte.

**Afficher aussi les touches de texte** étend la capture au texte ordinaire. Cette option peut révéler des informations privées et des mots de passe. L’application ne détecte pas les champs de mot de passe : arrêtez la capture avant toute saisie sensible, même dans le mode limité aux raccourcis.

### Souris et plusieurs écrans

Activez **Afficher les clics de souris** avant de démarrer. Les boutons gauche, droit et central ont leurs propres couleurs. Un clic très court reste visible au moins 150 ms ; un bouton maintenu reste coloré pendant un glisser. Le défilement de la molette n’est pas affiché.

Sous **Windows**, activez **Cercle au clic** avant de démarrer : cela active aussi **Afficher les clics de souris**. Le cercle fonctionne avec un seul écran, sélectionné automatiquement. Si vous débranchez les autres écrans, il suit automatiquement le seul écran restant. Avec plusieurs écrans, choisissez celui capturé dans OBS. Dans **Paramètres → Capture → Écran capturé dans OBS**, choisissez le moniteur utilisé dans OBS. Repérez-le grâce à sa résolution et à sa position sur le bureau ; les numéros DISPLAY ne correspondent pas nécessairement à l’ordre des sources OBS. Le choix peut changer pendant la capture. Déplacer l’incrustation clavier ne déplace pas le cercle du pointeur. Un clic sur un autre moniteur affiche le bouton, mais aucun cercle sur le moniteur choisi. Si celui-ci disparaît et que plusieurs écrans restent connectés, son cercle est suspendu jusqu’au choix d’un écran disponible ; clavier et boutons restent actifs.

Le cercle apparaît dans l’incrustation OBS, pas directement sur le bureau Windows. Son alignement exige des proportions et des rectangles identiques pour la capture du moniteur et la source Navigateur. Les captures de fenêtres et les recadrages ne sont pas compensés automatiquement.

Sous **Linux/Wayland**, les touches et boutons fonctionnent sur plusieurs écrans, mais le cercle à la position du pointeur n’est pas disponible.

## 7. Mémorisation et confidentialité

La position, les couleurs et le format sont enregistrés automatiquement. Les autres réglages — taille, durée, options de capture, moniteur choisi et langue — restent limités à la session dans cette alpha. En cas d’échec de sauvegarde, un message le signale dans les Paramètres ; les changements restent appliqués pour la session en cours.

Emplacements du fichier de préférences :

- Windows : `%APPDATA%\keycast-bridge\appearance.json`
- Linux : `$XDG_CONFIG_HOME/keycast-bridge/appearance.json`, ou `~/.config/keycast-bridge/appearance.json`

Ce fichier contient seulement l’apparence et le format : aucune frappe, aucun historique de souris, aucune URL OBS. Un fichier invalide entraîne un retour aux valeurs par défaut. Pour effacer toutes les préférences enregistrées, fermez l’application et renommez ce fichier, puis relancez-la.

Le serveur écoute uniquement sur l’adresse locale IPv4, au port 48732. Il n’y a ni télémétrie, ni historique des frappes, ni capture automatique au démarrage. L’URL aléatoire de session et les contrôles d’origine limitent l’accès, mais ne protègent pas contre un logiciel malveillant exécuté sous le même utilisateur. La capture ne s’arrête pas automatiquement au verrouillage de session : arrêtez-la avant. Sous Linux, effectuez l’arrêt d’urgence sur un clavier capturé, avec tous les modificateurs maintenus sur ce même clavier.

## 8. Dépannage

| Problème | Vérifications |
| --- | --- |
| La source Navigateur manque dans OBS Linux | Utilisez une version d’OBS qui l’inclut ; voir l’option Flatpak officiel dans le README. |
| L’incrustation est vide | Cliquez sur Tester le rendu ; vérifiez l’ordre et la visibilité des sources, ainsi que l’URL de la session. Le texte ordinaire est filtré par défaut. |
| OBS affiche une ancienne page ou une erreur | Recopiez l’URL après avoir relancé Keycast Bridge, puis actualisez la source Navigateur. |
| La position ou la taille diffère de l’aperçu | L’aperçu est schématique. Faites correspondre le format et les dimensions OBS ; vérifiez les recadrages, transformations et la Taille des touches. |
| Les boutons de souris sont absents | Arrêtez, cochez Afficher les clics de souris, puis redémarrez la capture. |
| Le cercle est décalé ou absent | Fonction Windows : activez-la, choisissez le bon écran, faites correspondre les proportions et alignez les sources sans recadrage. |
| L’autorisation de capture Linux échoue | Vérifiez le paquet/composant installé et la demande Polkit. Ne lancez pas l’interface en root. |
| Des réglages disparaissent au redémarrage | Seuls l’apparence et le format sont mémorisés. Vérifiez le message d’échec de sauvegarde et les droits du dossier de préférences. |
| L’exécutable Windows ne démarre pas | Extrayez tout le ZIP en gardant les DLL et dossiers. Ne copiez pas seulement l’EXE. |
| L’application se ferme immédiatement | Fermez une autre instance utilisant le port 48732. Si nécessaire, indiquez votre système et la version dans un rapport de bug. |

Pour un signalement utile, précisez la version de l’application, le système, le bureau et sa mise à l’échelle, le format choisi, les dimensions de la source OBS et les étapes de reproduction. Les captures d’écran sont utiles ; masquez l’URL de session et toute information privée affichée.

### Le guide ou l’aperçu ne s’ouvre pas

Sous Windows, le bouton utilise le navigateur par défaut du système. Si Windows refuse l’ouverture, une boîte de dialogue propose **Copier le lien** : collez-le dans votre navigateur et gardez Keycast Bridge ouvert. Vérifiez le navigateur par défaut dans les paramètres Windows. Le guide est aussi disponible hors ligne dans `guide/fr.html` à côté de l’exécutable.
