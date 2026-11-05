# Calculatrice (Rust + Tauri 2 + Svelte 5)

Calculatrice de bureau inspirée de **Windows Calculator**, conçue pour **Pop!_OS / Linux (Wayland)** avec **Rust** (`rust_decimal` 28 chiffres significatifs), **Tauri 2** et **Svelte 5** (Runes).

## Fonctionnalités

- **6 langues intégrées** (`🇫🇷 Français`, `🇬🇧 English`, `🇩🇪 Deutsch`, `🇮🇹 Italiano`, `🇪🇸 Español`, `🇵🇹 Português`) avec adaptation automatique des séparateurs décimaux/milliers, des messages d'erreur, des raccourcis et des étiquettes d'accessibilité.
- **Mode Standard** (`Alt+1`) : exécution immédiate (`2 + 3 × 4 = 20`), pourcentage contextuel (`200 + 10 % = 220`), `1/x`, `x²`, `√x`, répétition de `=`.
- **Mode Scientifique** (`Alt+2`) : priorité des opérations (`2 + 3 × 4 = 14`), parenthèses avec compteur d'imbrication, trigonométrie exacte aux angles remarquables (`DEG`, `RAD`, `GRAD`), modes `2nde` et `hyp`, puissances (`xʸ`, `10ˣ`, `2ˣ`, `eˣ`), racines (`√x`, `∛x`, `ʸ√x`), logarithmes (`log`, `ln`), factorielle (`n!`), valeur absolue (`|x|`), modulo (`mod`), constantes `π` et `e`.
- **Arithmétique décimale exacte en Rust** : `0,1 + 0,2 = 0,3` et `1 ÷ 3 × 3 = 1` sans erreur d'arrondi binaire.
- **Historique & Mémoire complète** (`MC`, `MR`, `M+`, `M−`, `MS`, `M▾`) : tiroir coulissant sur fenêtre compacte, panneau latéral automatique dès que la fenêtre est élargie (`≥ 640 px`).
- **Thèmes officiels Catppuccin** (`Latte`, `Frappé`, `Macchiato`, `Mocha`) + **7 couleurs d'accent Catppuccin** (`Mauve`, `Bleu`, `Saphir`, `Lavande`, `Pêche`, `Vert`, `Rose`), sauvegardés automatiquement dans le menu latéral **☰**.
- **Support intégral du pavé numérique sous Wayland** (`Numpad0`–`Numpad9`, `NumpadDecimal`, `NumpadAdd`, `NumpadSubtract`, `NumpadMultiply`, `NumpadDivide`, `NumpadEnter`, avec ou sans `Verr Num` activé).
- **Icône vectorielle et animée créée avec ArtCraft** (`VectorCraft` + `EffectCraft`, fichiers sources natifs dans [`assets/`](assets/)).
- **Empaquetage Flatpak natif & CI/CD GitHub Actions** ([`.github/workflows/flatpak.yml`](.github/workflows/flatpak.yml)).

## Commandes

```bash
# Tester le moteur de calcul Rust (17 suites de tests unitaires)
cargo test --manifest-path src-tauri/moteur/Cargo.toml

# Vérifier le typage Svelte 5 + TypeScript
npm run check

# Lancer l'application en mode développement (fenêtre native Wayland)
npm run tauri dev

# Compiler l'application optimisée, créer calculatrice.flatpak et l'installer dans Flatpak (--user)
npm run flatpak

# Lancer l'application Flatpak installée
flatpak run fr.tonyboysuper.Calculatrice
```

## Raccourcis clavier principaux

| Raccourci | Action |
| :--- | :--- |
| `0`–`9`, `,` ou `.` (clavier & pavé numérique) | Chiffres et séparateur décimal |
| `+`, `-`, `*`, `/` | Opérations de base |
| `Entrée` ou `=` | Calculer / répéter la dernière opération |
| `Échap` / `Suppr` / `Retour arrière` | Tout effacer (`C`) / Effacer l'entrée (`CE`) / Effacer un chiffre (`⌫`) |
| `Alt+1` / `Alt+2` | Mode Standard / Mode Scientifique |
| `H` | Ouvrir / fermer l'historique |
| `Ctrl+C` / `Ctrl+V` | Copier le résultat / Coller un nombre (formats FR et EN reconnus) |
| `Ctrl+L` / `Ctrl+R` / `Ctrl+M` / `Ctrl+P` / `Ctrl+Q` | `MC` / `MR` / `MS` / `M+` / `M−` |
| `F9`, `R`, `Q`, `@`, `%`, `!`, `^`, `(`, `)`, `P` | `+/−`, `1/x`, `x²`, `√x`, `%`, `n!`, `xʸ`, parenthèses, `π` |
