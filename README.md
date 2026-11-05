<div align="center">

<img src="src-tauri/icons/icon.svg" width="128" height="128" alt="Calculatrice Icon">

# Calculatrice

**A fast, modern desktop calculator for Linux & Wayland — inspired by Windows Calculator, powered by 128-bit exact Rust decimal math, and styled with official Catppuccin themes.**

[![Release](https://img.shields.io/github/v/release/Lib-LOCALE/calculatrice?label=release&color=cba6f7)](https://github.com/Lib-LOCALE/calculatrice/releases/latest)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Wayland%20%7C%20Flatpak-89b4fa)](https://github.com/Lib-LOCALE/calculatrice/releases/latest)
[![Stack](https://img.shields.io/badge/stack-Rust%20%2B%20Tauri%202%20%2B%20Svelte%205-fab387)](https://github.com/Lib-LOCALE/calculatrice)
[![Catppuccin](https://img.shields.io/badge/themes-Catppuccin-f5c2e7)](https://catppuccin.com)
[![License: MIT](https://img.shields.io/badge/license-MIT-a6e3a1.svg)](LICENSE)

[**Download Flatpak (`v0.1.0`)**](https://github.com/Lib-LOCALE/calculatrice/releases/latest) · [**Report a Bug**](https://github.com/Lib-LOCALE/calculatrice/issues) · [**Request a Feature**](https://github.com/Lib-LOCALE/calculatrice/issues)

<br>

<img src="docs/screenshot.png" width="820" alt="Calculatrice Scientific Mode & History in Catppuccin Mocha">

</div>

---

## Why Calculatrice?

Linux has plenty of calculators, yet switching from **Windows Calculator** to **GNOME Calculator**, **KCalc**, or **Galculator** often comes with trade-offs: missing immediate-execution Standard mode, inconsistent numpad handling under Wayland, rigid toolkit theming, or floating-point rounding artifacts.

**Calculatrice** brings the intuitive muscle memory of Windows Calculator to Linux and Wayland (`Pop!_OS`, `COSMIC`, `GNOME`, `KDE Plasma`, `Hyprland`, `Sway`) in a lightweight native binary built with **Rust**, **Tauri 2**, and **Svelte 5**:

- **True 128-bit Decimal Arithmetic (`rust_decimal`)** — Up to 28 significant digits with zero binary floating-point surprises: `0.1 + 0.2 = 0.3` and `1 ÷ 3 × 3 = 1`. Right-angle trigonometry is exact (`sin(180°) = 0`, `cos(90°) = 0`, `tan(45°) = 1`).
- **Dual Calculation Engines (Standard vs. Scientific)**
  - **Standard Mode (`Alt+1`)**: Classic immediate execution (`2 + 3 × 4 = 20`), contextual percentages (`200 + 10% = 220`), reciprocals (`1/x`), squares (`x²`), square roots (`√x`), and `=` operation repeat.
  - **Scientific Mode (`Alt+2`)**: Full operator precedence (`2 + 3 × 4 = 14`), nested parentheses with a live depth badge, `DEG` / `RAD` / `GRAD` angle units, `2nd` and hyperbolic (`hyp`) function layers, powers (`xʸ`, `10ˣ`, `2ˣ`, `eˣ`), roots (`√x`, `∛x`, `ʸ√x`), logarithms (`log`, `ln`), factorial (`n!`), modulo (`mod`), absolute value (`|x|`), and constants (`π`, `e`).
- **First-Class Wayland & Numpad Support** — Hardware scancode detection (`Numpad0`–`Numpad9`, `NumpadDecimal`, `NumpadComma`, `NumpadAdd`, `NumpadSubtract`, `NumpadMultiply`, `NumpadDivide`, `NumpadEnter`, plus XKB `KP_*` keys) ensures your numeric keypad always works on Wayland regardless of `NumLock` state. Includes automatic DMA-BUF fallback for NVIDIA Wayland sessions.
- **Official Catppuccin Palette Built-In** — Choose between all 4 official flavors (**Latte** ☀️, **Frappé**  frappe, **Macchiato** 🌺, **Mocha** 🌙) and **7 accent colors** (**Mauve**, **Blue**, **Sapphire**, **Lavender**, **Peach**, **Green**, **Pink**) directly from the `☰` settings drawer.
- **Adaptive Responsive Layout** — Works as a compact desktop calculator with a slide-up **History & Memory** drawer, and automatically expands into a two-column split view with a permanent sidebar as soon as the window is widened (`≥ 640 px`).
- **6 Languages with Locale-Aware Formatting** — Instant switching between **English** (`🇬🇧`), **Français** (`🇫🇷`), **Deutsch** (`🇩🇪`), **Italiano** (`🇮🇹`), **Español** (`🇪🇸`), and **Português** (`🇵🇹`), automatically adapting thousands/decimal separators, error diagnostics, and accessibility labels.

---

## Comparison with Other Desktop Calculators

| Feature | **Calculatrice** | Windows Calculator | GNOME Calculator | KCalc |
| :--- | :---: | :---: | :---: | :---: |
| **Native Linux / Wayland** | ✅ | ❌ (Windows only) | ✅ | ✅ |
| **128-bit Exact Decimal Math** | ✅ (`rust_decimal`) | ✅ | ✅ (MPFR) | ✅ (GMP/MPFR) |
| **Immediate-Execution Standard Mode** | ✅ (`2 + 3 × 4 = 20`) | ✅ | ❌ (Expression only) | ✅ |
| **Operator-Precedence Scientific Mode** | ✅ (`2 + 3 × 4 = 14`) | ✅ | ✅ | ✅ |
| **Adaptive Split-View History & Memory** | ✅ | ✅ | ❌ | ❌ |
| **Wayland Numpad (`NumLock` independent)** | ✅ | N/A | ⚠️ Compositor-dependent | ⚠️ Compositor-dependent |
| **Built-in Catppuccin Flavors & Accents** | ✅ (4 flavors × 7 accents) | ❌ | ❌ | ❌ |
| **In-App Instant Language Switcher** | ✅ (6 languages) | ❌ (OS locale) | ❌ (OS locale) | ✅ |

---

## Installation

### Option 1 — Flatpak Bundle (Recommended for Linux)

Download `calculatrice.flatpak` from the [**Latest Release**](https://github.com/Lib-LOCALE/calculatrice/releases/latest) and install it in your user Flatpak installation:

```bash
# Download and install the latest Flatpak bundle
curl -L -o calculatrice.flatpak https://github.com/Lib-LOCALE/calculatrice/releases/latest/download/calculatrice.flatpak
flatpak --user install -y --bundle calculatrice.flatpak

# Launch Calculatrice
flatpak run io.github.Lib_LOCALE.Calculatrice
```

> **Note:** Calculatrice uses the standard `org.gnome.Platform//50` runtime. If not already installed on your machine, install it via `flatpak install flathub org.gnome.Platform//50`.

### Option 2 — Build & Install from Source

Requires **Rust** (stable, `1.85+`), **Node.js** (`20+`), and **Flatpak**:

```bash
git clone https://github.com/Lib-LOCALE/calculatrice.git
cd calculatrice
npm install

# Run the 17 Rust calculation engine unit test suites
cargo test --manifest-path src-tauri/moteur/Cargo.toml

# Verify Svelte 5 + TypeScript types
npm run check

# Run in development mode
npm run tauri dev

# Build optimized release binary, package calculatrice.flatpak, and install to --user
npm run flatpak
```

---

## Keyboard Shortcuts

| Shortcut | Action |
| :--- | :--- |
| `0`–`9`, `.` or `,` (Main keyboard & Numpad) | Digits & decimal separator |
| `+`, `-`, `*`, `/` (Main keyboard & Numpad) | Basic arithmetic operations |
| `Enter`, `NumpadEnter`, or `=` | Evaluate expression / repeat last operation |
| `Escape` | Clear all (`C`) |
| `Delete` | Clear current entry (`CE`) |
| `Backspace` | Delete last digit (`⌫`) |
| `Alt+1` / `Alt+2` | Switch to **Standard** / **Scientific** mode |
| `H` | Toggle History drawer |
| `Ctrl+C` / `Ctrl+V` | Copy result / Paste number (parses both `.` and `,` formats) |
| `Ctrl+L` / `Ctrl+R` / `Ctrl+M` / `Ctrl+P` / `Ctrl+Q` | Memory: `MC` / `MR` / `MS` / `M+` / `M−` |
| `F9` | Toggle sign (`+/−`) |
| `R` / `Q` / `@` / `%` | Reciprocal (`1/x`) / Square (`x²`) / Square root (`√x`) / Percent (`%`) |
| `(` / `)` / `^` / `!` / `P` | Parentheses / Power (`xʸ`) / Factorial (`n!`) / Pi (`π`) |
| `S` / `O` / `T` / `L` / `N` | Scientific: `sin` / `cos` / `tan` / `log` / `ln` |

---

## Project Architecture

```text
calculatrice/
├── src-tauri/
│   ├── moteur/              # Pure Rust 128-bit decimal calculation engine + 17 test suites
│   ├── src/main.rs          # Tauri 2 native shell + Wayland/NVIDIA environment setup
│   └── icons/               # Multi-resolution PNGs + animated SVG app icon
├── src/
│   ├── App.svelte           # Responsive calculator UI, settings drawer & split-view panel
│   ├── app.css              # Official Catppuccin Latte, Frappé, Macchiato & Mocha palettes
│   └── lib/
│       ├── calc.svelte.ts   # Svelte 5 Runes reactive state store
│       ├── i18n.ts          # 6-language translations & locale-aware number formatter
│       ├── keys.ts          # Keypad definitions & Wayland-resilient keyboard/numpad mapper
│       └── LogoAnime.svelte # Theme-reactive animated vector icon component
├── assets/                  # Native VectorCraft (.vectorcraft) & EffectCraft (.ecproj) icon sources
├── flatpak/                 # Desktop entry, AppStream metainfo & Flatpak manifest (io.github.Lib_LOCALE.Calculatrice)
└── scripts/build-flatpak.sh # One-step release build & Flatpak bundle generator
```

---

## License

Released under the [MIT License](LICENSE). Contributions, bug reports, and feature requests are welcome!
