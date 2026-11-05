#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

APP_ID="io.github.Lib_LOCALE.Calculatrice"
OLD_APP_ID="fr.tonyboysuper.Calculatrice"
RUNTIME="org.gnome.Platform"
SDK="org.gnome.Platform"
BRANCH="${FLATPAK_RUNTIME_BRANCH:-50}"

BUILD_DIR="$ROOT_DIR/src-tauri/target/flatpak-build"
REPO_DIR="$ROOT_DIR/src-tauri/target/flatpak-repo"
BUNDLE_FILE="$ROOT_DIR/calculatrice.flatpak"

echo "1/4 Compilation du frontend Svelte 5..."
npm run build

echo "2/4 Compilation du binaire Rust optimisé (Tauri 2 release)..."
if [ -d "$ROOT_DIR/src-tauri/.sysroot/lib/pkgconfig" ]; then
  export PKG_CONFIG_PATH="$ROOT_DIR/src-tauri/.sysroot/lib/pkgconfig:${PKG_CONFIG_PATH:-}"
fi
npx tauri build --no-bundle

BIN_PATH="$ROOT_DIR/src-tauri/target/release/calculatrice"

echo "3/4 Assemblage du paquet Flatpak ($APP_ID, runtime $RUNTIME//$BRANCH)..."
rm -rf "$BUILD_DIR" "$REPO_DIR"
flatpak build-init "$BUILD_DIR" "$APP_ID" "$SDK" "$RUNTIME" "$BRANCH"

flatpak build "$BUILD_DIR" mkdir -p \
  /app/bin \
  /app/share/applications \
  /app/share/metainfo \
  /app/share/app-info/xmls \
  /app/share/app-info/icons/flatpak/64x64 \
  /app/share/app-info/icons/flatpak/128x128 \
  /app/share/icons/hicolor/scalable/apps \
  /app/share/icons/hicolor/32x32/apps \
  /app/share/icons/hicolor/64x64/apps \
  /app/share/icons/hicolor/128x128/apps \
  /app/share/icons/hicolor/256x256/apps \
  /app/share/icons/hicolor/512x512/apps

cp "$BIN_PATH" "$BUILD_DIR/files/bin/calculatrice"
chmod 755 "$BUILD_DIR/files/bin/calculatrice"

cp "$ROOT_DIR/flatpak/$APP_ID.desktop" "$BUILD_DIR/files/share/applications/$APP_ID.desktop"
cp "$ROOT_DIR/flatpak/$APP_ID.metainfo.xml" "$BUILD_DIR/files/share/metainfo/$APP_ID.metainfo.xml"

{
  echo '<?xml version="1.0" encoding="UTF-8"?>'
  echo '<components version="1.0" origin="flatpak">'
  sed '1d; s|</component>|  <icon type="cached" width="64" height="64">'"$APP_ID"'.png</icon>\n  <icon type="cached" width="128" height="128">'"$APP_ID"'.png</icon>\n  <icon type="stock">'"$APP_ID"'</icon>\n</component>|' "$ROOT_DIR/flatpak/$APP_ID.metainfo.xml"
  echo '</components>'
} | gzip -9 -n > "$BUILD_DIR/files/share/app-info/xmls/$APP_ID.xml.gz"

cp "$ROOT_DIR/src-tauri/icons/icon.svg" "$BUILD_DIR/files/share/icons/hicolor/scalable/apps/$APP_ID.svg"
cp "$ROOT_DIR/src-tauri/icons/32x32.png" "$BUILD_DIR/files/share/icons/hicolor/32x32/apps/$APP_ID.png"
if [ -f "$ROOT_DIR/src-tauri/icons/64x64.png" ]; then
  cp "$ROOT_DIR/src-tauri/icons/64x64.png" "$BUILD_DIR/files/share/icons/hicolor/64x64/apps/$APP_ID.png"
  cp "$ROOT_DIR/src-tauri/icons/64x64.png" "$BUILD_DIR/files/share/app-info/icons/flatpak/64x64/$APP_ID.png"
fi
cp "$ROOT_DIR/src-tauri/icons/128x128.png" "$BUILD_DIR/files/share/icons/hicolor/128x128/apps/$APP_ID.png"
cp "$ROOT_DIR/src-tauri/icons/128x128.png" "$BUILD_DIR/files/share/app-info/icons/flatpak/128x128/$APP_ID.png"
cp "$ROOT_DIR/src-tauri/icons/128x128@2x.png" "$BUILD_DIR/files/share/icons/hicolor/256x256/apps/$APP_ID.png"
cp "$ROOT_DIR/src-tauri/icons/icon.png" "$BUILD_DIR/files/share/icons/hicolor/512x512/apps/$APP_ID.png"

flatpak build-finish "$BUILD_DIR" \
  --command=calculatrice \
  --socket=wayland \
  --socket=fallback-x11 \
  --share=ipc \
  --device=dri \
  --env=WEBKIT_DISABLE_DMABUF_RENDERER=1

flatpak build-export "$REPO_DIR" "$BUILD_DIR"
flatpak build-bundle --runtime-repo=https://dl.flathub.org/repo/flathub.flatpakrepo "$REPO_DIR" "$BUNDLE_FILE" "$APP_ID"

if [ "${FLATPAK_SKIP_INSTALL:-0}" != "1" ]; then
  echo "4/4 Installation dans le Flatpak utilisateur et intégration COSMIC..."
  flatpak --user uninstall -y --delete-data "$OLD_APP_ID" >/dev/null 2>&1 || true
  flatpak --user uninstall -y --delete-data "$APP_ID" >/dev/null 2>&1 || true
  flatpak --user remote-delete --force calculatrice-origin >/dev/null 2>&1 || true
  flatpak --user remote-delete --force calculatrice-local-origin >/dev/null 2>&1 || true
  rm -rf "$HOME/.local/share/fr.tonyboysuper.calculatrice" "$HOME/.cache/fr.tonyboysuper.calculatrice" 2>/dev/null || true

  flatpak --user install -y --bundle "$BUNDLE_FILE"

  # Index AppStream metadata for COSMIC Store (calculatrice-origin)
  APPSTREAM_DIR="$HOME/.local/share/flatpak/appstream/calculatrice-origin/x86_64/active"
  mkdir -p "$APPSTREAM_DIR/icons/64x64" "$APPSTREAM_DIR/icons/128x128"
  zcat "$BUILD_DIR/files/share/app-info/xmls/$APP_ID.xml.gz" > "$APPSTREAM_DIR/appstream.xml"
  cp "$BUILD_DIR/files/share/app-info/xmls/$APP_ID.xml.gz" "$APPSTREAM_DIR/appstream.xml.gz"
  if [ -f "$ROOT_DIR/src-tauri/icons/64x64.png" ]; then
    cp "$ROOT_DIR/src-tauri/icons/64x64.png" "$APPSTREAM_DIR/icons/64x64/$APP_ID.png"
  fi
  cp "$ROOT_DIR/src-tauri/icons/128x128.png" "$APPSTREAM_DIR/icons/128x128/$APP_ID.png"
  rm -rf "$HOME/.cache/cosmic-store/calculatrice-origin" "$HOME/.cache/cosmic-store/explore_cache.bin.zst" 2>/dev/null || true
fi

echo "Terminé ! Paquet généré : $BUNDLE_FILE"
