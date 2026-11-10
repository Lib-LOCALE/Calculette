<p align="center"><img src="icons/icon.svg" alt="Calculette" width="112"></p>
<h1 align="center">Calculette</h1>

<p align="center">A native standard and scientific calculator for Linux and Windows.<br>Written in Rust, built with Slint, powered by decimal arithmetic.</p>

<p align="center">
  <a href="https://github.com/Lib-LOCALE/Calculette/actions/workflows/ci.yml"><img src="https://github.com/Lib-LOCALE/Calculette/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/code-MIT-78e3cb" alt="Code: MIT"></a>
  <a href="https://github.com/Lib-LOCALE/Calculette/releases"><img src="https://img.shields.io/github/v/release/Lib-LOCALE/Calculette?include_prereleases" alt="Latest release"></a>
</p>

![Calculette — scientific mode and history](docs/screenshot.png)

## What you can do

- Calculate in **Standard** mode with immediate execution (`2 + 3 × 4 = 20`), or **Scientific** mode with operator precedence (`2 + 3 × 4 = 14`).
- Use decimal arithmetic for basic calculations: `0.1 + 0.2 = 0.3`. Scientific functions include nested parentheses, DEG/RAD/GRAD, inverse and hyperbolic trigonometry, powers, roots, logarithms, factorial, modulo, π and e.
- Recall calculations from **history**, store values in **memory** and manage individual entries. Wide windows show a permanent panel; compact windows use a drawer.
- Choose **Catppuccin** (Mocha, Macchiato, Frappé or Latte), **Tokyo Night, Dracula, Nord or Gruvbox**, with seven accent colours.
- Work in **French, English, German, Italian, Spanish or Portuguese**, with local number formatting, keyboard navigation and translated accessibility labels. Physical numpad keys are recognised independently of NumLock.

## Install

The native Slint version is **0.2.0 on `main`**. A 0.2.0 release has not been published yet; the release badge above follows the versions actually available on [Releases](https://github.com/Lib-LOCALE/Calculette/releases).

Build from source below, or create and install the Linux Flatpak:

```sh
bash scripts/build-flatpak.sh
flatpak run io.github.Lib_LOCALE.Calculatrice
```

See [Flatpak build and installation instructions](flatpak/README.md) for prerequisites and bundle-only builds.

## Build from source

Use Rust **1.92 or newer** and a C/C++ build toolchain.

### Linux (Debian/Ubuntu)

```sh
git clone https://github.com/Lib-LOCALE/Calculette.git
cd Calculette
# Debian/Ubuntu build dependencies
sudo apt install build-essential pkg-config libfontconfig1-dev libfreetype6-dev \
  libxkbcommon-dev libwayland-dev libx11-dev libxcb1-dev libegl1-mesa-dev
cargo build --release --locked
./target/release/calculatrice
```

### Windows (10 / 11 x64)

Prerequisites: Rust **1.92 or newer** and a C/C++ toolchain (MSVC or MinGW-w64).

```powershell
git clone https://github.com/Lib-LOCALE/Calculette.git
cd Calculette
cargo build --release
.\target\release\calculatrice.exe
```

Or run the helper script:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\build-windows.ps1 -Run
```

The executable renders directly through Slint and Winit. If a GPU renderer is unavailable:

```sh
SLINT_BACKEND=winit-software ./target/release/calculatrice
```

## Everyday shortcuts

| Action | Shortcut | Action | Shortcut |
| --- | --- | --- | --- |
| Standard / scientific | Alt+1 / Alt+2 | History drawer | H |
| Evaluate / repeat | Enter or = | Clear all | Esc |
| Clear entry | Delete | Delete digit | Backspace |
| Copy / paste number | Ctrl+C / Ctrl+V | Negate | F9 |
| Memory clear / recall | Ctrl+L / Ctrl+R | Memory store | Ctrl+M |
| Memory add / subtract | Ctrl+P / Ctrl+Q | Reciprocal / square | R / Q |
| Square root / percent | @ / % | Power / factorial | ^ / ! |
| sin / cos / tan | S / O / T | log / ln | L / N |

Numbers accept `.` or `,`. Scientific shortcuts apply in scientific mode. When settings are open, keyboard input goes to their controls.

Language, theme and accent use `$XDG_CONFIG_HOME/io.github.Lib_LOCALE.Calculatrice/preferences.json` (`~/.config/…` by default on Linux, `%APPDATA%\io.github.Lib_LOCALE.Calculatrice\preferences.json` on Windows). Choose these once after migrating from the previous interface; its old settings are retained on disk. History and memory last for the current session.

## Project status

**0.2.0 native port.** Linux/Wayland, X11, and Windows 10/11 x64 are supported platforms. The suite has **22 tests**; see [validation scope and limitations](docs/validation/slint-migration.md).

Basic arithmetic uses `rust_decimal`. Transcendental functions use `f64`, rounded to 15 significant digits. Comparative RAM/startup performance, clipboard behaviour across compositors and screen-reader operation have not been benchmarked or comprehensively tested.

## Development

```sh
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --locked -p calculatrice --no-deps --all-targets -- -D warnings
```

The application and calculation engine are Rust; the interface is `.slint`. Two small development helpers live in `scripts/`: Shell builds/installs Flatpak bundles, and Python 3.11+ regenerates offline Cargo sources after dependency changes. **Neither is an application runtime dependency.** A native Cargo build does not use either helper.

Report reproducible problems through [Issues](https://github.com/Lib-LOCALE/Calculette/issues).

## Credits and licences

- **Application code:** [MIT](LICENSE).
- **Slint:** used under its [Royalty-free Desktop, Mobile, and Web Applications License 2.0](https://github.com/slint-ui/slint/blob/v1.18.1/LICENSES/LicenseRef-Slint-Royalty-free-2.0.md). The settings menu includes `AboutSlint`.
- **Font:** Inter, [SIL Open Font License](ui/fonts/OFL.txt).
- **Branding:** original editable VectorCraft and EffectCraft sources in [assets](assets).

<p align="center"><a href="https://slint.dev"><img src="https://raw.githubusercontent.com/slint-ui/slint/master/logo/MadeWithSlint-logo-whitebg.png" alt="Made with Slint" width="160"></a></p>
