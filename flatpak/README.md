# Linux Flatpak

Calculette uses `org.freedesktop.Platform//26.08` and the matching SDK. Its application ID remains `io.github.Lib_LOCALE.Calculatrice`.

Install Flatpak and either `flatpak-builder` or the `org.flatpak.Builder` app. Configure the Flathub user remote:

```sh
flatpak --user remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
```

From the repository root:

```sh
# Compile inside the SDK, create the bundle and install/update the user application
bash scripts/build-flatpak.sh

# Create calculatrice.flatpak without installing it
FLATPAK_SKIP_INSTALL=1 bash scripts/build-flatpak.sh

flatpak run io.github.Lib_LOCALE.Calculatrice
```

The SDK build avoids dependence on the host's newer glibc. Installation updates preserve user data. The sandbox allows Wayland, fallback X11, shared IPC and GPU rendering.

## Offline dependencies

`cargo-sources.json` lists the registry archives and checksums from the committed `Cargo.lock`. Flatpak downloads these archives before compiling; Cargo then builds offline with `--locked`.

After changing Rust dependencies, regenerate the list using Python 3.11 or newer:

```sh
python3 scripts/generate-cargo-sources.py
```

Python is used only for this maintenance command. The Shell helper orchestrates Flatpak tools; neither interpreter is used by the application.

The manifest in this repository builds the local checkout. Downstream packaging repositories pin the published Git commit instead. Changes to the application repository do not automatically update downstream PRs. Consult each distribution's contribution policy before preparing or submitting packaging.
