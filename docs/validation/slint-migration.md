# Validation du portage Slint — 6 novembre 2026

Le moteur original est déplacé de `src-tauri/moteur` vers `moteur`. L'interface est compilée avec Slint 1.18.1, Winit, FemtoVG et l'accessibilité activée. Les traductions, palettes et couches de touches sont reprises des données de l'interface précédente.

## Vérifications

- `cargo test --locked --workspace` : 21 tests réussis (17 moteur, 4 interface native).
- `cargo clippy --locked -p calculatrice --no-deps --all-targets -- -D warnings` : réussi. Le moteur conserve ses avertissements de style Clippy préexistants ; son comportement n'est pas modifié.
- `cargo fmt --all -- --check`, `bash -n scripts/build-flatpak.sh`, `git diff --check` : réussis.
- Tests graphiques exécutés exclusivement sous `gpu-run`, avec configuration temporaire et locale française.
- Wayland natif (`env -u DISPLAY … target/debug/calculatrice --smoke-test`) et X11/XWayland (`env -u WAYLAND_DISPLAY …`) : calcul décimal 0,1 + 0,2, priorité scientifique, historique, stockage et rappel mémoire, thème Latte, localisation allemande, six langues, erreur et reprise de saisie.
- Captures inspectées aux formats 380×640, 820×640 et 320×540 : pavé, panneau latéral, réglages clairs, tiroir mémoire et message d'erreur.
- Le test recrée la surface native entre les tailles, car le compositeur Wayland ignore les demandes programmatiques de redimensionnement d'une fenêtre déjà affichée. Sous X11, le redimensionnement direct a également été vérifié pendant le portage. L'interface réagit aux changements de largeur via Slint, sans dépendre d'un filtre d'événements Winit.
- Construction Flatpak hors ligne dans le SDK Freedesktop 25.08, à partir de Cargo.lock et des archives vérifiées : bundle `calculatrice.flatpak` (environ 5,3 Mo).
- Le binaire du paquet démarre dans `org.freedesktop.Platform//25.08` avec le socket Wayland et reste actif jusqu’à l’arrêt de contrôle par timeout (code 124). Dans ce contexte `flatpak build --runtime`, le bus des réglages desktop n’est pas exposé et produit un avertissement ; installation utilisateur et intégration au menu non retestées. Les bibliothèques liées ne comprennent ni WebKitGTK ni Tauri.
- Licences MIT et OFL de la police Inter incluses dans `/app/share/licenses/calculatrice/`.

## Limites et publication

Le logo des réglages est statique ; l’ancienne vidéo de démonstration n’a pas été refaite. Pas de mesure comparative de RAM ou de démarrage. Pas de test automatisé du presse-papiers ni de lecteur d'écran. Les anciens réglages WebKit ne sont pas importés automatiquement ; ils ne sont pas supprimés. Le menu conserve l'attribution AboutSlint et la licence MIT des sources est conservée.

Aucun push, aucune release GitHub et aucune modification de la soumission Flathub existante. Le manifeste utilise le dossier source local ; une soumission Flathub doit référencer le commit publié contenant le portage.
