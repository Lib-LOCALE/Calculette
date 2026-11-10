use crate::{CalculatorWindow, Palette};
use slint::winit_030::WinitWindowAccessor;
use slint::{ComponentHandle, Model};

// Debug-only UI exercise: real window, callbacks and renderer, no production automation endpoint.
pub fn run(ui: &CalculatorWindow) {
    let weak = ui.as_weak();
    slint::Timer::single_shot(std::time::Duration::from_millis(800), move || {
        let ui = weak.unwrap();
        for id in ["clear", "d0", "dec", "d1", "add", "d0", "dec", "d2", "eq"] {
            ui.invoke_press(id.into());
        }
        assert_eq!(ui.get_display().as_str(), "0,3");
        ui.invoke_press("store".into());
        assert!(ui.get_has_memory());
        ui.invoke_press("scientifique".into());
        assert!(ui.get_scientific());
        for id in ["clear", "d2", "add", "d3", "mul", "d4", "eq"] {
            ui.invoke_press(id.into());
        }
        assert_eq!(ui.get_display().as_str(), "14");
        assert_eq!(ui.get_history().row_count(), 2);
        step(ui.as_weak(), 0);
    });
}
fn step(weak: slint::Weak<CalculatorWindow>, index: usize) {
    slint::Timer::single_shot(std::time::Duration::from_millis(400), move || {
        let ui = weak.unwrap();
        ui.window().with_winit_window(|w| {
            eprintln!(
                "step {index} native={:?} resizable={} maximized={} slint={:?}",
                w.inner_size(),
                w.is_resizable(),
                w.is_maximized(),
                ui.window().size()
            )
        });
        let names = ["compact", "wide", "latte-de", "settings", "memory", "error"];
        let buffer = ui.window().take_snapshot().expect("capture du rendu natif");
        let prefix = std::env::var("CALCULETTE_SCREENSHOT").unwrap_or_else(|_| {
            std::env::temp_dir()
                .join("calculette-slint")
                .to_string_lossy()
                .into_owned()
        });
        let path = format!("{prefix}-{}.ppm", names[index]);
        let mut bytes = format!("P6\n{} {}\n255\n", buffer.width(), buffer.height()).into_bytes();
        for pixel in buffer.as_slice() {
            bytes.extend_from_slice(&[pixel.r, pixel.g, pixel.b]);
        }
        std::fs::write(&path, bytes).unwrap();
        match index {
            0 => resize(&ui, 820.0, 640.0),
            1 => {
                assert!(ui.get_wide());
                ui.set_theme_index(3);
                ui.set_language_index(2);
                ui.invoke_configure();
                assert_eq!(
                    ui.global::<Palette>().get_bg(),
                    crate::native::color("#eff1f5")
                );
                for id in ["clear", "d1", "d2", "d3", "d4", "dec", "d5"] {
                    ui.invoke_press(id.into());
                }
                assert_eq!(ui.get_display().as_str(), "1.234,5");
            }
            2 => ui.set_settings(true),
            3 => {
                ui.set_settings(false);
                ui.set_language_index(0);
                ui.set_theme_index(0);
                ui.invoke_configure();
                ui.invoke_recall(true, 0);
                assert_eq!(ui.get_display().as_str(), "0,3");
                ui.invoke_press("standard".into());
                resize(&ui, 320.0, 540.0);
                ui.set_drawer(true);
                ui.set_memory_tab(true);
            }
            4 => {
                assert!(!ui.get_wide());
                ui.set_drawer(false);
                for id in ["clear", "d1", "div", "d0", "eq"] {
                    ui.invoke_press(id.into());
                }
                assert!(ui.get_error());
                assert_eq!(ui.get_display().as_str(), "Division par zéro impossible");
            }
            _ => {
                for language in 0..6 {
                    ui.set_language_index(language);
                    ui.invoke_configure();
                    assert!(!ui.get_display().is_empty());
                }
                println!(
                    "UI smoke OK: exact decimal, precedence, history, memory recall, theme, six languages, compact/wide/drawer, error recovery; snapshots {prefix}-*.ppm"
                );
                slint::quit_event_loop().unwrap();
                return;
            }
        }
        step(ui.as_weak(), index + 1);
    });
}

fn resize(ui: &CalculatorWindow, width: f32, height: f32) {
    // Recreate the native surface: some Wayland compositors ignore mapped resize requests.
    ui.hide().unwrap();
    ui.window().set_size(slint::LogicalSize::new(width, height));
    ui.show().unwrap();
}
