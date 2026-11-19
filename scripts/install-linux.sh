#!/usr/bin/env bash
set -euo pipefail

cd -- "$(dirname -- "$0")"
app_id=io.github.Lib_LOCALE.Calculatrice
binary="$HOME/.local/bin/calculatrice"
data_home="${XDG_DATA_HOME:-$HOME/.local/share}"

install -Dm755 calculatrice "$binary"
install -Dm644 icons/icon.svg "$data_home/icons/hicolor/scalable/apps/$app_id.svg"
for size in 32 64 128; do
    install -Dm644 "icons/${size}x${size}.png" "$data_home/icons/hicolor/${size}x${size}/apps/$app_id.png"
done
install -Dm644 icons/128x128@2x.png "$data_home/icons/hicolor/256x256/apps/$app_id.png"
install -Dm644 icons/icon.png "$data_home/icons/hicolor/512x512/apps/$app_id.png"

# A quoted desktop-entry Exec argument undergoes both string and argument unescaping.
exec_path=$(printf '%s' "$binary" | sed -e 's/[%]/%%/g' -e 's/[\\"`$]/\\&/g' -e 's/\\/\\\\/g')
launcher="$data_home/applications/$app_id.desktop"
mkdir -p -- "$(dirname -- "$launcher")"
while IFS= read -r line; do
    if [[ $line == Exec=* ]]; then
        printf 'Exec="%s"\n' "$exec_path"
    else
        printf '%s\n' "$line"
    fi
done < "$app_id.desktop" > "$launcher"

for notice in LICENSE OFL.txt LICENSE-flag-icons.txt; do
    install -Dm644 "$notice" "$data_home/licenses/calculatrice/$notice"
done
printf 'Calculette is ready in your application launcher.\n'
