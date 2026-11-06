#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod native;
#[cfg(debug_assertions)]
mod smoke;

use moteur::{Angle, Engine, Key, Mode};
use native::{Data, Preferences};
use slint::winit_030::{EventResult, WinitWindowAccessor, winit};
use slint::{ComponentHandle, ModelRc, VecModel};
use std::{cell::RefCell, rc::Rc};
use winit::{
    event::{ElementState, WindowEvent},
    keyboard::{Key as NativeKey, ModifiersState, NamedKey},
};

slint::include_modules!();

struct App {
    engine: Engine,
    data: Data,
    preferences: Preferences,
    second: bool,
    hyperbolic: bool,
}

impl App {
    fn refresh(&self, ui: &CalculatorWindow) {
        let state = self.engine.snapshot();
        let tr = &self.data.languages[self.preferences.language];
        let display = match state.error_kind {
            Some(error) => tr["erreurs"][serde_json::to_value(error).unwrap().as_str().unwrap()]
                .as_str()
                .unwrap()
                .to_owned(),
            None => native::format(&state.display, tr),
        };
        ui.set_display_length(display.chars().count() as i32);
        ui.set_display(display.into());
        ui.set_expression(native::format(&state.expression, tr).into());
        ui.set_error(state.error);
        ui.set_scientific(state.mode == Mode::Scientifique);
        ui.set_has_memory(!state.memory.is_empty());
        ui.set_angle(
            match state.angle {
                Angle::Deg => "DEG",
                Angle::Rad => "RAD",
                Angle::Grad => "GRAD",
            }
            .into(),
        );
        ui.set_open_parens(state.open_parens as i32);
        ui.set_history(ModelRc::new(VecModel::from(
            state
                .history
                .iter()
                .map(|v| Record {
                    expression: native::format(&v.expression, tr).into(),
                    result: native::format(&v.result, tr).into(),
                })
                .collect::<Vec<_>>(),
        )));
        ui.set_memory(strings(state.memory.iter().map(|s| native::format(s, tr))));
        let rows = self
            .data
            .rows(state.mode, self.second, self.hyperbolic)
            .into_iter()
            .map(|row| {
                ModelRc::new(VecModel::from(
                    row.into_iter()
                        .map(|key| {
                            let label = match key.id.as_str() {
                                "dec" => tr["separateurDecimal"].as_str().unwrap(),
                                "seconde" => tr["toucheSeconde"].as_str().unwrap(),
                                _ => &key.label,
                            };
                            CalcKey {
                                id: key.id.clone().into(),
                                label: native::superscript(label).into(),
                                aria: tr["touches"][&key.id].as_str().unwrap_or(&key.aria).into(),
                                kind: match key.genre.as_str() {
                                    "chiffre" => 1,
                                    "egal" => 2,
                                    _ => 0,
                                },
                                enabled: !state.error || key.sure,
                                active: (key.id == "seconde" && self.second)
                                    || (key.id == "hyp" && self.hyperbolic),
                            }
                        })
                        .collect::<Vec<_>>(),
                ))
            })
            .collect::<Vec<_>>();
        ui.set_keys(ModelRc::new(VecModel::from(rows)));
        ui.set_accents(strings(
            native::ACCENTS
                .iter()
                .map(|id| tr["accents"][id].as_str().unwrap().to_owned()),
        ));
        let palette = ui.global::<Palette>();
        let colors = &self.data.themes[self.preferences.theme]["colors"];
        let color = |key: &str| native::color(colors[key].as_str().unwrap());
        palette.set_bg(color("ctp-base"));
        palette.set_panel(color("ctp-mantle"));
        palette.set_number(if self.preferences.theme == 3 {
            native::color("#ffffff")
        } else {
            color("ctp-surface1")
        });
        palette.set_function(if self.preferences.theme == 3 {
            color("ctp-mantle")
        } else {
            color("ctp-surface0")
        });
        palette.set_hover(color("ctp-surface2"));
        palette.set_text(color("ctp-text"));
        palette.set_muted(color("ctp-subtext0"));
        palette.set_accent(color(&format!(
            "ctp-{}",
            native::ACCENTS[self.preferences.accent]
        )));
        palette.set_on_accent(color("on-accent"));
        palette.set_red(color("ctp-red"));
    }

    fn press(&mut self, id: &str) {
        match id {
            "standard" | "scientifique" => {
                self.engine.press(Key::SetMode(if id == "standard" {
                    Mode::Standard
                } else {
                    Mode::Scientifique
                }));
                self.second = false;
                self.hyperbolic = false;
            }
            "angle" => {
                self.engine
                    .press(Key::SetAngle(match self.engine.snapshot().angle {
                        Angle::Deg => Angle::Rad,
                        Angle::Rad => Angle::Grad,
                        Angle::Grad => Angle::Deg,
                    }));
            }
            "store" => {
                self.engine.press(Key::MemoryStore);
            }
            "history-clear" => {
                self.engine.press(Key::HistoryClear);
            }
            "seconde" => self.second = !self.second,
            "hyp" => self.hyperbolic = !self.hyperbolic,
            _ => {
                if let Some(key) = self.data.find_key(
                    id,
                    self.engine.snapshot().mode,
                    self.second,
                    self.hyperbolic,
                ) && (!self.engine.snapshot().error || key.sure)
                    && let Some(key) = key.key
                {
                    self.engine.press(key);
                }
            }
        }
    }
}

fn strings(values: impl IntoIterator<Item = String>) -> ModelRc<slint::SharedString> {
    ModelRc::new(VecModel::from(
        values.into_iter().map(Into::into).collect::<Vec<_>>(),
    ))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    slint::BackendSelector::new()
        .backend_name("winit".into())
        .select()?;
    let ui = CalculatorWindow::new()?;
    let data = Data::load();
    let preferences = Preferences::load(&data);
    ui.set_language_index(preferences.language as i32);
    ui.set_theme_index(preferences.theme as i32);
    ui.set_accent_index(preferences.accent as i32);
    ui.set_languages(strings(
        data.languages
            .iter()
            .map(|l| l["nom"].as_str().unwrap().to_owned()),
    ));
    ui.set_themes(strings(
        data.themes
            .iter()
            .map(|t| native::theme_name(t["id"].as_str().unwrap()).to_owned()),
    ));
    let app = Rc::new(RefCell::new(App {
        engine: Engine::new(),
        data,
        preferences,
        second: false,
        hyperbolic: false,
    }));
    app.borrow().refresh(&ui);

    let state = app.clone();
    let weak = ui.as_weak();
    ui.on_press(move |id| {
        let ui = weak.unwrap();
        let mut app = state.borrow_mut();
        app.press(&id);
        app.refresh(&ui);
    });
    let languages = app.borrow().data.languages.clone();
    ui.on_translate(move |key, language| {
        let tr = &languages[language.max(0) as usize % languages.len()];
        tr["ui"][key.as_str()]
            .as_str()
            .or_else(|| tr["modes"][key.as_str()].as_str())
            .unwrap_or(key.as_str())
            .into()
    });
    let state = app.clone();
    let weak = ui.as_weak();
    ui.on_configure(move || {
        let ui = weak.unwrap();
        let mut app = state.borrow_mut();
        app.preferences.language =
            (ui.get_language_index().max(0) as usize).min(app.data.languages.len() - 1);
        app.preferences.theme =
            (ui.get_theme_index().max(0) as usize).min(app.data.themes.len() - 1);
        app.preferences.accent =
            (ui.get_accent_index().max(0) as usize).min(native::ACCENTS.len() - 1);
        if let Err(error) = app.preferences.save(&app.data) {
            eprintln!("Préférences : {error}");
        }
        app.refresh(&ui);
    });
    let state = app.clone();
    let weak = ui.as_weak();
    ui.on_recall(move |memory, index| {
        if index < 0 {
            return;
        }
        let ui = weak.unwrap();
        let mut app = state.borrow_mut();
        app.engine.press(if memory {
            Key::MemoryRecall(Some(index as usize))
        } else {
            Key::HistoryRecall(index as usize)
        });
        ui.set_drawer(false);
        app.refresh(&ui);
    });
    let state = app.clone();
    let weak = ui.as_weak();
    ui.on_memory_action(move |action, index| {
        let ui = weak.unwrap();
        let mut app = state.borrow_mut();
        if !app.engine.snapshot().error
            && let Some(key) = native::memory_key(&action, index)
        {
            app.engine.press(key);
        }
        app.refresh(&ui);
    });
    let clipboard = Rc::new(RefCell::new(arboard::Clipboard::new().ok()));
    let clip = clipboard.clone();
    let weak = ui.as_weak();
    ui.on_copy(move || {
        let ui = weak.unwrap();
        if ui.get_error() {
            return;
        }
        if let Some(clipboard) = clip.borrow_mut().as_mut() {
            match clipboard.set_text(ui.get_display().replace('\u{202f}', "")) {
                Ok(()) => {
                    ui.set_copied(true);
                    let weak = ui.as_weak();
                    slint::Timer::single_shot(std::time::Duration::from_millis(1200), move || {
                        if let Some(ui) = weak.upgrade() {
                            ui.set_copied(false);
                        }
                    });
                }
                Err(error) => eprintln!("Copie : {error}"),
            }
        }
    });
    let state = app.clone();
    let weak = ui.as_weak();
    let mut modifiers = ModifiersState::empty();
    ui.window().on_winit_window_event(move |_, event| {
        if let WindowEvent::ModifiersChanged(value) = event {
            modifiers = value.state();
        }
        let WindowEvent::KeyboardInput { event, .. } = event else {
            return EventResult::Propagate;
        };
        if event.state != ElementState::Pressed {
            return EventResult::Propagate;
        }
        let ui = weak.unwrap();
        if ui.get_settings() {
            if event.logical_key == NativeKey::Named(NamedKey::Escape) {
                ui.set_settings(false);
                return EventResult::PreventDefault;
            }
            return EventResult::Propagate;
        }
        if event.logical_key == NativeKey::Named(NamedKey::Escape) && ui.get_drawer() {
            ui.set_drawer(false);
            return EventResult::PreventDefault;
        }
        let action = native::keyboard(event, modifiers, &state.borrow().data.shortcuts);
        let Some(action) = action else {
            return EventResult::Propagate;
        };
        match action.as_str() {
            "copy" => ui.invoke_copy(),
            "paste" => {
                if let Some(clipboard) = clipboard.borrow_mut().as_mut() {
                    match clipboard.get_text() {
                        Ok(text) => {
                            state.borrow_mut().engine.press(Key::Paste(text));
                        }
                        Err(error) => eprintln!("Coller : {error}"),
                    }
                }
            }
            "history" => {
                ui.set_memory_tab(false);
                ui.set_drawer(!ui.get_drawer());
            }
            "memoryClear" | "memoryRecall" | "memoryAdd" | "memorySub" => {
                ui.invoke_memory_action(action.clone().into(), -1)
            }
            id => state.borrow_mut().press(id),
        }
        state.borrow().refresh(&ui);
        EventResult::PreventDefault
    });
    #[cfg(debug_assertions)]
    if std::env::args().any(|a| a == "--smoke-test") {
        smoke::run(&ui);
        ui.show()?;
        slint::run_event_loop_until_quit()?;
        return Ok(());
    }
    ui.run()?;
    Ok(())
}
