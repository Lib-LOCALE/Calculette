#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"
APP_ID="io.github.Lib_LOCALE.Calculatrice"
mkdir -p target

# Compile inside the SDK so the binary never depends on the host's newer glibc.
if command -v flatpak-builder >/dev/null; then
  BUILDER=(flatpak-builder)
else
  BUILDER=(flatpak run org.flatpak.Builder)
fi
"${BUILDER[@]}" --user --force-clean --install-deps-from=flathub \
  --repo=target/flatpak-repo target/flatpak-build "flatpak/$APP_ID.json"
flatpak build-bundle --runtime-repo=https://dl.flathub.org/repo/flathub.flatpakrepo \
  target/flatpak-repo calculatrice.flatpak "$APP_ID"
if [[ "${FLATPAK_SKIP_INSTALL:-0}" != 1 ]]; then
  flatpak --user install -y --or-update --bundle calculatrice.flatpak
fi
printf 'Paquet généré : %s/calculatrice.flatpak\n' "$ROOT_DIR"
