# Keycast Bridge — Guide d’utilisation

Version **0.2.0** · [English](USER_GUIDE.en.md)

Keycast Bridge affiche les raccourcis clavier, les modificateurs maintenus et, en option, les clics de souris dans une source Navigateur OBS transparente. L’application contrôle la capture et l’apparence ; OBS superpose le résultat à la capture de votre écran ou logiciel.

## 1. Installer et lancer

### Windows x64

Téléchargez le ZIP Windows dans les Releases GitHub du projet. Extrayez **toute l’archive** dans un dossier accessible en écriture, puis ouvrez `keycast-bridge.exe`. Conservez les DLL et les sous-dossiers à côté de l’exécutable. Il n’y a ni installateur, ni pilote, ni service permanent. Lancez l’application en utilisateur normal. Cette version n’est pas signée ; les bureaux sécurisés, les fenêtres UAC et les logiciels exécutés en administrateur ne font pas partie de la cible de capture prise en charge.

### Debian 13 amd64

Téléchargez le paquet Debian depuis les Releases, ouvrez un terminal dans son dossier et lancez :

```sh
sudo apt install ./keycast-bridge_0.2.0-1_amd64.deb
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
| Arrêter | Terminer la capture, effacer l’incrustation et arrêter les outils du bureau. |
| Ctrl + Alt + F12 | Arrêt d’urgence. Les outils du bureau peuvent intercepter ce raccourci lorsqu’ils sont actifs. |
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
| Accent / clic gauche | Le contour des touches maintenues et le bouton gauche. |
| Clic droit | La mise en évidence du bouton droit. |
| Clic molette | La mise en évidence du bouton central. |

Cliquez sur une pastille pour ouvrir le sélecteur de couleur. **Palette sombre** et **Palette claire** rétablissent un ensemble de couleurs en conservant la position, les dimensions et le style de l’anneau. **Tout réinitialiser**, dans cet onglet, réinitialise la position, les couleurs, l’anneau et les styles de présentation : le format et les options de capture restent inchangés.

## 5. Paramètres — Format et taille

**Taille** règle le texte des touches entre 20 et 96 pixels du format logique. **Durée** règle l’affichage du dernier raccourci entre 300 et 5000 ms. Les modificateurs maintenus restent visibles jusqu’au relâchement, quelle que soit cette durée.

Les formats prédéfinis sont 1280 × 720, 1920 × 1080, 2560 × 1440, 3840 × 2160, 1080 × 1920 et 1080 × 1080. Une largeur et une hauteur personnalisées peuvent chacune aller de 160 à 7680 pixels.

Ces valeurs définissent le **format logique de l’incrustation** et modifient son aperçu et son rendu. **Elles ne changent pas à distance les propriétés de la source Navigateur d’OBS.** Reportez les mêmes dimensions dans OBS pour obtenir le rendu prévu. Si la fenêtre du navigateur a d’autres dimensions, le format est réduit ou agrandi uniformément pour y tenir ; une différence de proportions laisse des marges transparentes, sans déformation. La taille apparente des touches peut donc varier si les dimensions ne correspondent pas.

Exemple : pour un tutoriel vertical en 1080 × 1920, choisissez ce format dans l’application, réglez la source Navigateur OBS sur 1080 × 1920 et alignez-la avec votre scène verticale. Pour le cercle au clic Windows sur une capture de moniteur entier, choisissez un format ayant les proportions de cet écran et alignez les deux sources sans recadrage.

## 6. Paramètres — Capture

### Claviers et disposition

Sous **Linux**, le réglage par défaut capture tous les claviers reconnus, y compris ceux branchés ensuite. Vous pouvez sélectionner des claviers précis dans les Paramètres et actualiser la liste. Choisissez une disposition correspondant à votre bureau : AZERTY France, QWERTY États-Unis ou Royaume-Uni, QWERTZ Allemagne. Débrancher un clavier efface son état de touches maintenues. En mode automatique, rebrancher ou remplacer un clavier ne demande pas de redémarrer OBS.

Sous **Windows**, tous les claviers de la session sont capturés et la disposition suit la fenêtre active. Cette version ne propose pas de sélection individuelle des claviers Windows.

### Raccourcis, texte et modificateurs

Par défaut, les caractères de texte ordinaires sont filtrés. Des raccourcis tels que Ctrl+C et des touches de contrôle nommées peuvent néanmoins apparaître. **Maj (`Shift`), Ctrl, Alt, Win/Super et AltGr** sont affichés dans une ligne dédiée tant qu’ils sont maintenus, même sans autre touche du clavier. Les variantes gauche/droite sont regroupées : relâcher un côté ne masque pas le modificateur encore maintenu de l’autre côté.

Cela permet notamment d’utiliser la Plume de Photoshop/Affinity : maintenez Maj, Ctrl ou Alt, puis cliquez ou glissez. Activez **Afficher les clics de souris** pour voir les boutons à côté des modificateurs. Le dernier raccourci reste dans une ligne temporaire distincte.

**Afficher aussi les touches de texte** étend la capture au texte ordinaire. Cette option peut révéler des informations privées et des mots de passe. L’application ne détecte pas les champs de mot de passe : arrêtez la capture avant toute saisie sensible, même dans le mode limité aux raccourcis.

### Souris et plusieurs écrans

Activez **Afficher les clics de souris** avant de démarrer. Les boutons gauche, droit et central ont leurs propres couleurs. Un clic très court reste visible au moins 150 ms ; un bouton maintenu reste coloré pendant un glisser. Le défilement de la molette n’est pas affiché.

Sous **Windows**, activez **Cercle au clic** avant de démarrer : cela active aussi **Afficher les clics de souris**. Le cercle fonctionne avec un seul écran, sélectionné automatiquement. Si vous débranchez les autres écrans, il suit automatiquement le seul écran restant. Avec plusieurs écrans, choisissez celui capturé dans OBS. Dans **Paramètres → Capture → Écran capturé dans OBS**, choisissez le moniteur utilisé dans OBS. Repérez-le grâce à sa résolution et à sa position sur le bureau ; les numéros DISPLAY ne correspondent pas nécessairement à l’ordre des sources OBS. Le choix peut changer pendant la capture. Déplacer l’incrustation clavier ne déplace pas le cercle du pointeur. Un clic sur un autre moniteur affiche le bouton, mais aucun cercle sur le moniteur choisi. Si celui-ci disparaît et que plusieurs écrans restent connectés, son cercle est suspendu jusqu’au choix d’un écran disponible ; clavier et boutons restent actifs.

Le cercle apparaît dans l’incrustation OBS, pas directement sur le bureau Windows. Son alignement exige des proportions et des rectangles identiques pour la capture du moniteur et la source Navigateur. Les captures de fenêtres et les recadrages ne sont pas compensés automatiquement.

Sous **Linux/Wayland**, les touches et boutons fonctionnent sur plusieurs écrans. La source Navigateur ne reçoit pas de coordonnées du pointeur ; GNOME 48 propose séparément l’anneau du bureau via **Présentation** et la capture d’écran.

### Anneau au clic — Windows

Ouvrez **Paramètres → Anneau** pour régler :

| Réglage | Fonction |
| --- | --- |
| Couleur | Couleur propre à l’anneau, indépendante des touches et des boutons de souris. |
| Rayon de base | De 10 à 200 pixels du canevas OBS ; indépendant de la taille des touches. |
| Épaisseur | De 1 à 20 pixels, sans dépasser le rayon. |
| Effet néon | Ajoute une lueur intérieure et extérieure de la couleur choisie. |
| Effet pulsation | Agrandit l’anneau de 70 à 140 % du rayon de base pendant sa disparition. Désactivé : rayon fixe avec fondu. |

Chaque clic affiche l’anneau pendant 0,5 seconde. Le rayon et l’épaisseur suivent la mise à l’échelle du canevas. L’aperçu réduit se répète sans démarrer la capture. Les modifications s’appliquent en direct et sont mémorisées. **Réinitialiser l’anneau** restaure le violet, un rayon de 30 px, une épaisseur de 3 px, sans néon et avec pulsation.

Activez **Cercle au clic** dans la fenêtre principale pour l’afficher dans OBS. Il reste lié au moniteur choisi et apparaît au clic, pas continuellement autour du curseur. Cette fonction n’ajoute pas de halo global sous Linux.

## 7. Mémorisation et confidentialité

La position, les couleurs, le format, l’anneau et les styles de présentation sont enregistrés automatiquement. Les autres réglages — taille, durée, options de capture, moniteur choisi et langue — restent limités à la session dans cette version. En cas d’échec de sauvegarde, un message le signale dans les Paramètres ; les changements restent appliqués pour la session en cours.

Emplacements du fichier de préférences :

- Windows : `%APPDATA%\keycast-bridge\appearance.json`
- Linux : `$XDG_CONFIG_HOME/keycast-bridge/appearance.json`, ou `~/.config/keycast-bridge/appearance.json`

Ce fichier contient seulement l’apparence et le format : aucune frappe, aucun historique de souris, aucune URL OBS. Un fichier invalide entraîne un retour aux valeurs par défaut. Pour effacer toutes les préférences enregistrées, fermez l’application et renommez ce fichier, puis relancez-la.

Le serveur écoute uniquement sur l’adresse locale IPv4, au port 48732. Il n’y a ni télémétrie, ni historique des frappes, ni capture automatique au démarrage. L’URL aléatoire de session et les contrôles d’origine limitent l’accès, mais ne protègent pas contre un logiciel malveillant exécuté sous le même utilisateur. La capture du clavier seule ne s’arrête pas automatiquement au verrouillage de session : arrêtez-la avant. Les outils de présentation actifs ajoutent un arrêt au verrouillage. Sous Linux, effectuez l’arrêt d’urgence sur un clavier capturé, avec tous les modificateurs maintenus sur ce même clavier.

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


## Outils de présentation sur le bureau

Ouvrez **Paramètres → Présentation**, puis activez les **outils sur le bureau**. Ils restent **arrêtés à chaque lancement** de Keycast Bridge. Ils fonctionnent indépendamment de la capture du clavier. Le bouton **Arrêter** de la fenêtre principale arrête aussi les outils du bureau.

Sous Windows 10/11, aucune installation supplémentaire n’est nécessaire. Sous Debian 13 avec GNOME 48/Wayland, activez l’extension GNOME **Keycast Bridge presentation tools** fournie. Après installation du paquet Debian, fermez puis rouvrez votre session, puis exécutez :

```sh
gnome-extensions enable keycast-bridge@jeromelab.fr
```

Pour une installation depuis les sources, lancez `./scripts/install-gnome-extension.sh` avec votre utilisateur habituel, puis fermez/rouvrez la session et activez l’extension. Les autres compositeurs et versions de GNOME ne sont pas encore pris en charge pour ces outils du bureau ; l’incrustation du clavier reste indépendante. Désactiver l’extension supprime immédiatement ses effets. Fermez Keycast Bridge avant de désinstaller le paquet.

### Effets du pointeur

- **Anneau :** choisissez la couleur, le rayon (10–200 px), l’épaisseur (1–20 px), le néon et la pulsation dans **Paramètres → Anneau**. Activez l’anneau continu dans **Présentation**. Un anneau bref marque aussi les clics. Les mêmes réglages d’apparence s’appliquent à l’anneau optionnel de la source OBS.
- **Traînée :** six styles sont proposés : Plein, Tiret, Point, Lueur, Particule et Étoile. La couleur suit celle de l’anneau ; les styles en lignes utilisent aussi son épaisseur ; les points disparaissent en 0,65 seconde.
- **Secouer pour localiser :** faites rapidement plusieurs allers-retours avec la souris. Un grand anneau apparaît brièvement. Un déplacement rapide en ligne droite ne le déclenche pas.
- **Spotlight :** assombrissez l’écran courant autour d’une zone éclairée en **cercle ou carré**. Réglez le rayon (40–600 px ; demi-côté pour le carré) et l’assombrissement (0–90 %).

Ces effets suivent le pointeur d’un écran à l’autre et laissent les applications utilisables normalement. Windows utilise les pixels physiques ; GNOME utilise les pixels logiques du bureau. La mise à l’échelle peut donc modifier la taille physique apparente.

### Dessin et tableau blanc

Placez le pointeur sur l’écran souhaité, puis pressez **Ctrl+Alt+D** pour annoter ou **Ctrl+Alt+B** pour afficher un fond blanc. Les boutons des paramètres ouvrent ces modes sur l’écran situé sous le pointeur. Choisissez la couleur et l’épaisseur du trait (1–40 px) dans Présentation.

| Action | Commande |
| --- | --- |
| Dessiner | Maintenir le bouton gauche et déplacer la souris |
| Annuler le dernier trait | Ctrl+Z |
| Tout effacer | Retour arrière |
| Quitter le mode et effacer les dessins | Échap |
| Arrêter tous les outils du bureau | Ctrl+Alt+F12 |

Ces modes utilisent la souris et le clavier tant qu’ils sont actifs. L’application située dessous ne reçoit pas les clics de dessin. L’écran ciblé reste fixe jusqu’à la sortie du mode. Les dessins sont temporaires : aucun export ni enregistrement automatique. La mémoire est limitée à 128 traits avec un nombre de points borné ; les traits très longs sont simplifiés.

### Zoom figé

Placez le pointeur sur l’écran souhaité et pressez **Ctrl+Alt+Z**, ou cliquez sur **Zoom**. Keycast Bridge prend une image de cet écran en mémoire et l’agrandit. **L’image est figée : ce n’est pas une loupe en direct.** Les animations et changements de l’application n’apparaissent qu’après avoir quitté puis relancé le zoom.

Déplacez le pointeur pour naviguer ; utilisez la molette ou **+ / −** pour modifier le grossissement (1,25×–8×). Le facteur initial est réglable. **Échap** revient au bureau et supprime l’image de la mémoire de l’application. Le dessin utilise les modes séparés annotation et tableau blanc.

### OBS, raccourcis et changements de session

Utilisez la **capture d’écran** OBS sous Windows ou la **capture d’écran / PipeWire** sous GNOME pour inclure ces effets. La capture d’une fenêtre et la seule source Navigateur du clavier ne les incluent pas. Lorsque vous capturez l’anneau du bureau, désactivez le **Cercle au clic** de la source Navigateur OBS pour éviter deux anneaux superposés.

Les raccourcis du bureau sont enregistrés uniquement lorsque les outils sont actifs. Si Windows signale un conflit, fermez ou reconfigurez l’utilitaire qui utilise déjà ces touches avant de réactiver les outils. Sous GNOME, les raccourcis peuvent être modifiés via le schéma GSettings de l’extension. Des touches réservées par le compositeur ou une autre application peuvent entrer en conflit.

La fermeture de l’application, l’arrêt des outils, le verrouillage de la session ou un changement de disposition des écrans suppriment les effets. GNOME les arrête aussi à l’ouverture de la Vue d’ensemble. Il faut ensuite les réactiver explicitement. Si l’application de contrôle disparaît, l’extension GNOME s’arrête en deux secondes maximum ; sous Windows, la perte du focus quitte le mode dessin/zoom. Quand les outils de présentation sont désactivés, la capture du clavier conserve ses limites de sécurité habituelles : arrêtez-la avant de saisir des secrets.

Les effets ciblent les sessions de bureau ordinaires ; les contenus protégés, bureaux sécurisés et applications en plein écran exclusif peuvent ne pas apparaître ou accepter les surimpressions. Aucun historique du pointeur, dessin ou cliché du zoom n’est enregistré sur disque ou transmis au serveur OBS. Seuls les réglages d’apparence sont mémorisés.
