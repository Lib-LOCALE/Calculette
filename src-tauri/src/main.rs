// Masque la console sous Windows en version finale (sans effet sous Linux).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use moteur::{Engine, Key, Snapshot};
use tauri::State;

type Moteur<'a> = State<'a, Mutex<Engine>>;

/// Transmet une touche au moteur et renvoie le nouvel état à afficher.
#[tauri::command]
fn appuyer(touche: Key, moteur: Moteur<'_>) -> Snapshot {
    moteur.lock().unwrap_or_else(|e| e.into_inner()).press(touche)
}

/// État courant, lu au démarrage de l'interface.
#[tauri::command]
fn etat(moteur: Moteur<'_>) -> Snapshot {
    moteur.lock().unwrap_or_else(|e| e.into_inner()).snapshot()
}

fn main() {
    // Pilote NVIDIA + WebKitGTK sous Wayland : le rendu DMA-BUF peut laisser la fenêtre vide.
    if std::path::Path::new("/proc/driver/nvidia").exists()
        && std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none()
    {
        // SAFETY : appelé au tout début du programme, avant la création de tout autre thread.
        unsafe { std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1") };
    }

    tauri::Builder::default()
        .manage(Mutex::new(Engine::new()))
        .invoke_handler(tauri::generate_handler![appuyer, etat])
        .run(tauri::generate_context!())
        .expect("impossible de lancer la Calculatrice");
}
