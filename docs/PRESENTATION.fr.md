# Outils de présentation sur le bureau

Ouvrez **Paramètres → Présentation**, puis activez les **outils sur le bureau**. Ils restent **arrêtés à chaque lancement** de Keycast Bridge. Ils fonctionnent indépendamment de la capture du clavier. Le bouton **Arrêter** de la fenêtre principale arrête aussi les outils du bureau.

Sous Windows 10/11, aucune installation supplémentaire n’est nécessaire. Sous Debian 13 avec GNOME 48/Wayland, activez l’extension GNOME **Keycast Bridge presentation tools** fournie. Après installation du paquet Debian, fermez puis rouvrez votre session, puis exécutez :

```sh
gnome-extensions enable keycast-bridge@jeromelab.fr
```

Pour une installation depuis les sources, lancez `./scripts/install-gnome-extension.sh` avec votre utilisateur habituel, puis fermez/rouvrez la session et activez l’extension. Les autres compositeurs et versions de GNOME ne sont pas encore pris en charge pour ces outils du bureau ; l’incrustation du clavier reste indépendante. Désactiver ou désinstaller l’extension supprime immédiatement ses effets.

## Effets du pointeur

- **Anneau :** choisissez la couleur, le rayon (10–200 px), l’épaisseur (1–20 px), le néon et la pulsation dans **Paramètres → Anneau**. Activez l’anneau continu dans **Présentation**. Un anneau bref marque aussi les clics. Les mêmes réglages d’apparence s’appliquent à l’anneau optionnel de la source OBS.
- **Traînée :** six styles sont proposés : Plein, Tiret, Point, Lueur, Particule et Étoile. La couleur et l’épaisseur suivent celles de l’anneau ; les points disparaissent en 0,65 seconde.
- **Secouer pour localiser :** faites rapidement plusieurs allers-retours avec la souris. Un grand anneau apparaît brièvement. Un déplacement rapide en ligne droite ne le déclenche pas.
- **Spotlight :** assombrissez l’écran courant autour d’une zone éclairée en **cercle ou carré**. Réglez le rayon (40–600 px ; demi-côté pour le carré) et l’assombrissement (0–90 %).

Ces effets suivent le pointeur d’un écran à l’autre et laissent les applications utilisables normalement. Windows utilise les pixels physiques ; GNOME utilise les pixels logiques du bureau. La mise à l’échelle peut donc modifier la taille physique apparente.

## Dessin et tableau blanc

Placez le pointeur sur l’écran souhaité, puis pressez **Ctrl+Alt+D** pour annoter ou **Ctrl+Alt+B** pour afficher un fond blanc. Les boutons des paramètres ouvrent ces modes sur l’écran situé sous le pointeur. Choisissez la couleur et l’épaisseur du trait (1–40 px) dans Présentation.

| Action | Commande |
| --- | --- |
| Dessiner | Maintenir le bouton gauche et déplacer la souris |
| Annuler le dernier trait | Ctrl+Z |
| Tout effacer | Retour arrière |
| Quitter le mode et effacer les dessins | Échap |
| Arrêter tous les outils du bureau | Ctrl+Alt+F12 |

Ces modes utilisent la souris et le clavier tant qu’ils sont actifs. L’application située dessous ne reçoit pas les clics de dessin. L’écran ciblé reste fixe jusqu’à la sortie du mode. Les dessins sont temporaires : aucun export ni enregistrement automatique. La mémoire est limitée à 128 traits avec un nombre de points borné ; les traits très longs sont simplifiés.

## Zoom figé

Placez le pointeur sur l’écran souhaité et pressez **Ctrl+Alt+Z**, ou cliquez sur **Zoom**. Keycast Bridge prend une image de cet écran en mémoire et l’agrandit. **L’image est figée : ce n’est pas une loupe en direct.** Les animations et changements de l’application n’apparaissent qu’après avoir quitté puis relancé le zoom.

Déplacez le pointeur pour naviguer ; utilisez la molette ou **+ / −** pour modifier le grossissement (1,25×–8×). Le facteur initial est réglable. **Échap** revient au bureau et supprime l’image de la mémoire de l’application. Le dessin utilise les modes séparés annotation et tableau blanc.

## OBS, raccourcis et changements de session

Utilisez la **capture d’écran** OBS sous Windows ou la **capture d’écran / PipeWire** sous GNOME pour inclure ces effets. La capture d’une fenêtre et la seule source Navigateur du clavier ne les incluent pas. Lorsque vous capturez l’anneau du bureau, désactivez le **Cercle au clic** de la source Navigateur OBS pour éviter deux anneaux superposés.

Les raccourcis du bureau sont enregistrés uniquement lorsque les outils sont actifs. Si Windows signale un conflit, fermez ou reconfigurez l’utilitaire qui utilise déjà ces touches avant de réactiver les outils. Sous GNOME, les raccourcis peuvent être modifiés via le schéma GSettings de l’extension. Des touches réservées par le compositeur ou une autre application peuvent entrer en conflit.

La fermeture de l’application, l’arrêt des outils, le verrouillage de la session ou un changement de disposition des écrans suppriment les effets. GNOME les arrête aussi à l’ouverture de la Vue d’ensemble. Il faut ensuite les réactiver explicitement. Si l’application de contrôle disparaît, l’extension GNOME s’arrête en deux secondes maximum ; sous Windows, la perte du focus quitte le mode dessin/zoom. Quand les outils de présentation sont désactivés, la capture du clavier conserve ses limites de sécurité habituelles : arrêtez-la avant de saisir des secrets.

Les effets ciblent les sessions de bureau ordinaires ; les contenus protégés, bureaux sécurisés et applications en plein écran exclusif peuvent ne pas apparaître ou accepter les surimpressions. Aucun historique du pointeur, dessin ou cliché du zoom n’est enregistré sur disque ou transmis au serveur OBS. Seuls les réglages d’apparence sont mémorisés.
