use moteur::{Key, Mode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use slint::winit_030::winit::{
    event::KeyEvent,
    keyboard::{Key as NativeKey, KeyCode, ModifiersState, NamedKey, PhysicalKey},
};
use std::{collections::HashMap, path::PathBuf};

pub const ACCENTS: [&str; 7] = [
    "mauve", "blue", "sapphire", "lavender", "peach", "green", "pink",
];
#[derive(Clone, Deserialize)]
pub struct Button {
    pub id: String,
    pub label: String,
    pub aria: String,
    pub genre: String,
    pub key: Option<Key>,
    #[serde(default)]
    pub sure: bool,
}
#[derive(Deserialize)]
struct Cell {
    base: Button,
    seconde: Option<Button>,
    hyp: Option<Button>,
    #[serde(rename = "hypSeconde")]
    hyp_seconde: Option<Button>,
}
impl Cell {
    fn resolve(&self, second: bool, hyp: bool) -> &Button {
        if hyp
            && second
            && let Some(key) = &self.hyp_seconde
        {
            return key;
        }
        if hyp && let Some(key) = &self.hyp {
            return key;
        }
        if second && let Some(key) = &self.seconde {
            return key;
        }
        &self.base
    }
}
#[derive(Deserialize)]
struct Keys {
    standard: Vec<Vec<Cell>>,
    scientifique: Vec<Vec<Cell>>,
    shortcuts: HashMap<String, String>,
}
pub struct Data {
    pub languages: Vec<Value>,
    pub themes: Vec<Value>,
    pub shortcuts: HashMap<String, String>,
    keys: Keys,
}
impl Data {
    pub fn load() -> Self {
        let keys: Keys = serde_json::from_str(include_str!("../ui/data/keys.json"))
            .expect("touches embarquées valides");
        Self {
            languages: serde_json::from_str(include_str!("../ui/data/languages.json"))
                .expect("langues valides"),
            themes: serde_json::from_str(include_str!("../ui/data/themes.json"))
                .expect("thèmes valides"),
            shortcuts: keys.shortcuts.clone(),
            keys,
        }
    }
    fn cells(&self, mode: Mode) -> &Vec<Vec<Cell>> {
        if mode == Mode::Standard {
            &self.keys.standard
        } else {
            &self.keys.scientifique
        }
    }
    pub fn rows(&self, mode: Mode, second: bool, hyp: bool) -> Vec<Vec<Button>> {
        self.cells(mode)
            .iter()
            .map(|r| r.iter().map(|c| c.resolve(second, hyp).clone()).collect())
            .collect()
    }
    pub fn find_key(&self, id: &str, mode: Mode, second: bool, hyp: bool) -> Option<Button> {
        self.cells(mode).iter().flatten().find_map(|c| {
            let resolved = c.resolve(second, hyp);
            if resolved.id == id {
                Some(resolved.clone())
            } else if c.base.id == id {
                Some(c.base.clone())
            } else {
                None
            }
        })
    }
}

pub fn format(text: &str, language: &Value) -> String {
    text.chars()
        .map(|c| match c {
            '\u{202f}' => language["separateurMilliers"].as_str().unwrap().to_owned(),
            ',' => language["separateurDecimal"].as_str().unwrap().to_owned(),
            _ => c.to_string(),
        })
        .collect()
}
pub fn superscript(text: &str) -> String {
    let mut result = String::new();
    let mut exponent = false;
    for c in text.chars() {
        match c {
            '{' => exponent = true,
            '}' => exponent = false,
            _ => result.push(if exponent {
                match c {
                    '0' => '⁰',
                    '1' => '¹',
                    '2' => '²',
                    '3' => '³',
                    'x' => 'ˣ',
                    'y' => 'ʸ',
                    '−' => '⁻',
                    _ => c,
                }
            } else {
                c
            }),
        }
    }
    result
}
pub fn color(hex: &str) -> slint::Color {
    let rgb =
        u32::from_str_radix(hex.trim_start_matches('#'), 16).expect("couleur embarquée valide");
    slint::Color::from_rgb_u8((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)
}
pub fn theme_name(id: &str) -> &str {
    match id {
        "mocha" => "🌙 Mocha",
        "macchiato" => "🌙 Macchiato",
        "frappe" => "🌙 Frappé",
        "latte" => "☀️ Latte",
        "tokyo-night" => "🌃 Tokyo Night",
        "dracula" => "🧛 Dracula",
        "nord" => "❄️ Nord",
        "gruvbox" => "🍂 Gruvbox",
        _ => id,
    }
}
pub fn memory_key(action: &str, index: i32) -> Option<Key> {
    let index = usize::try_from(index).ok();
    Some(match action {
        "memoryClear" => Key::MemoryClear(index),
        "memoryRecall" => Key::MemoryRecall(index),
        "memoryAdd" => Key::MemoryAdd(index),
        "memorySub" => Key::MemorySub(index),
        _ => return None,
    })
}

// Physical keys preserve the existing NumLock-independent Wayland keypad behavior.
pub fn numpad(code: KeyCode) -> Option<&'static str> {
    Some(match code {
        KeyCode::Numpad0 => "d0",
        KeyCode::Numpad1 => "d1",
        KeyCode::Numpad2 => "d2",
        KeyCode::Numpad3 => "d3",
        KeyCode::Numpad4 => "d4",
        KeyCode::Numpad5 => "d5",
        KeyCode::Numpad6 => "d6",
        KeyCode::Numpad7 => "d7",
        KeyCode::Numpad8 => "d8",
        KeyCode::Numpad9 => "d9",
        KeyCode::NumpadDecimal | KeyCode::NumpadComma => "dec",
        KeyCode::NumpadAdd => "add",
        KeyCode::NumpadSubtract => "sub",
        KeyCode::NumpadMultiply => "mul",
        KeyCode::NumpadDivide => "div",
        KeyCode::NumpadEnter | KeyCode::NumpadEqual => "eq",
        KeyCode::NumpadParenLeft => "open",
        KeyCode::NumpadParenRight => "close",
        KeyCode::NumpadBackspace => "back",
        KeyCode::NumpadClear => "ce",
        _ => return None,
    })
}
pub fn keyboard(
    event: &KeyEvent,
    mods: ModifiersState,
    shortcuts: &HashMap<String, String>,
) -> Option<String> {
    keyboard_action(&event.logical_key, event.physical_key, mods, shortcuts)
}
fn keyboard_action(
    logical: &NativeKey,
    physical: PhysicalKey,
    mods: ModifiersState,
    shortcuts: &HashMap<String, String>,
) -> Option<String> {
    let text = match logical {
        NativeKey::Character(s) => s.as_str(),
        NativeKey::Named(n) => match n {
            NamedKey::Enter => "Enter",
            NamedKey::Escape => "Escape",
            NamedKey::Delete => "Delete",
            NamedKey::Backspace => "Backspace",
            NamedKey::F9 => "F9",
            _ => "",
        },
        _ => "",
    };
    if (mods.control_key() || mods.super_key()) && !mods.alt_key() {
        return Some(
            match text.to_lowercase().as_str() {
                "c" => "copy",
                "v" => "paste",
                "l" => "memoryClear",
                "r" => "memoryRecall",
                "m" => "store",
                "p" => "memoryAdd",
                "q" => "memorySub",
                _ => return None,
            }
            .into(),
        );
    }
    if mods.alt_key() && !mods.control_key() && !mods.super_key() {
        return match physical {
            PhysicalKey::Code(KeyCode::Digit1) => Some("standard".into()),
            PhysicalKey::Code(KeyCode::Digit2) => Some("scientifique".into()),
            _ => None,
        };
    }
    if mods.control_key() || mods.super_key() || mods.alt_key() {
        return None;
    }
    if let PhysicalKey::Code(code) = physical
        && let Some(id) = numpad(code)
    {
        return Some(id.into());
    }
    if text.eq_ignore_ascii_case("h") {
        return Some("history".into());
    }
    shortcuts.get(text).cloned()
}

#[cfg(target_os = "windows")]
pub fn attach_console() {
    unsafe {
        unsafe extern "system" {
            fn AttachConsole(dwProcessId: u32) -> i32;
        }
        const ATTACH_PARENT_PROCESS: u32 = 0xFFFFFFFF;
        AttachConsole(ATTACH_PARENT_PROCESS);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CliOptions {
    pub language: Option<usize>,
    pub theme: Option<usize>,
    pub accent: Option<usize>,
    pub mode: Option<Mode>,
    pub smoke_test: bool,
    pub help: bool,
    pub version: bool,
}

impl CliOptions {
    pub fn parse(args: &[String], data: &Data) -> Result<Self, String> {
        let mut opts = Self::default();
        let mut iter = args.iter().peekable();
        let _ = iter.next();

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--help" | "-h" => opts.help = true,
                "--version" | "-v" => opts.version = true,
                "--smoke-test" => opts.smoke_test = true,
                "--lang" | "--language" | "-l" => {
                    let val = iter
                        .next()
                        .ok_or_else(|| "Option requise pour --lang".to_owned())?;
                    let val_lower = val.to_lowercase();
                    let idx = data
                        .languages
                        .iter()
                        .position(|l| {
                            l["code"].as_str().unwrap().eq_ignore_ascii_case(&val_lower)
                                || l["nom"].as_str().unwrap().to_lowercase() == val_lower
                        })
                        .ok_or_else(|| {
                            format!(
                                "Langue inconnue '{val}'. Valeurs possibles: fr, en, de, it, es, pt"
                            )
                        })?;
                    opts.language = Some(idx);
                }
                "--theme" | "-t" => {
                    let val = iter
                        .next()
                        .ok_or_else(|| "Option requise pour --theme".to_owned())?;
                    let val_lower = val.to_lowercase();
                    let idx = if let Ok(i) = val.parse::<usize>()
                        && i < data.themes.len()
                    {
                        i
                    } else {
                        data.themes
                            .iter()
                            .position(|t| {
                                t["id"].as_str().unwrap().eq_ignore_ascii_case(&val_lower)
                            })
                            .ok_or_else(|| {
                                format!(
                                    "Thème inconnu '{val}'. Valeurs possibles: mocha, macchiato, frappe, latte, tokyo-night, dracula, nord, gruvbox (ou 0..7)"
                                )
                            })?
                    };
                    opts.theme = Some(idx);
                }
                "--accent" | "-a" => {
                    let val = iter
                        .next()
                        .ok_or_else(|| "Option requise pour --accent".to_owned())?;
                    let val_lower = val.to_lowercase();
                    let idx = if let Ok(i) = val.parse::<usize>()
                        && i < ACCENTS.len()
                    {
                        i
                    } else {
                        ACCENTS
                            .iter()
                            .position(|a| a.eq_ignore_ascii_case(&val_lower))
                            .ok_or_else(|| {
                                format!(
                                    "Accent inconnu '{val}'. Valeurs possibles: mauve, blue, sapphire, lavender, peach, green, pink (ou 0..6)"
                                )
                            })?
                    };
                    opts.accent = Some(idx);
                }
                "--mode" | "-m" => {
                    let val = iter
                        .next()
                        .ok_or_else(|| "Option requise pour --mode".to_owned())?;
                    match val.to_lowercase().as_str() {
                        "standard" | "std" => opts.mode = Some(Mode::Standard),
                        "scientifique" | "scientific" | "sci" => {
                            opts.mode = Some(Mode::Scientifique)
                        }
                        _ => {
                            return Err(format!(
                                "Mode inconnu '{val}'. Valeurs possibles: standard, scientifique"
                            ));
                        }
                    }
                }
                unknown => {
                    return Err(format!(
                        "Option inconnue '{unknown}'. Utilisez --help pour voir la liste des options."
                    ));
                }
            }
        }
        Ok(opts)
    }

    pub fn print_help() {
        println!(
            "Calculette 0.2.0 - Calculette native standard et scientifique\n\n\
            UTILISATION :\n    \
                calculatrice [OPTIONS]\n\n\
            OPTIONS :\n    \
                -m, --mode <MODE>        Mode de départ : 'standard' ou 'scientifique'\n    \
                -t, --theme <THEME>      Thème : mocha, macchiato, frappe, latte, tokyo-night, dracula, nord, gruvbox (ou 0..7)\n    \
                -a, --accent <ACCENT>    Couleur d'accent : mauve, blue, sapphire, lavender, peach, green, pink (ou 0..6)\n    \
                -l, --lang <LANG>        Langue de l'interface : fr, en, de, it, es, pt\n    \
                -h, --help               Afficher cette aide\n    \
                -v, --version            Afficher la version\n        \
                    --smoke-test         Exécuter le test automatisé d'interface graphique"
        );
    }
}

#[derive(Default)]
pub struct Preferences {
    pub language: usize,
    pub theme: usize,
    pub accent: usize,
}
#[derive(Serialize, Deserialize)]
struct SavedPreferences {
    language: String,
    theme: String,
    accent: String,
}
fn preferences_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("USERPROFILE")
                    .map(|p| PathBuf::from(p).join("AppData").join("Roaming"))
            })
            .map(|p| {
                p.join("io.github.Lib_LOCALE.Calculatrice")
                    .join("preferences.json")
            })
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
            .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".config")))
            .map(|p| p.join("io.github.Lib_LOCALE.Calculatrice/preferences.json"))
    }
}
impl Preferences {
    pub fn load(data: &Data) -> Self {
        let locale = ["LC_ALL", "LC_MESSAGES", "LANG"]
            .iter()
            .filter_map(|k| std::env::var(k).ok())
            .find(|s| !s.is_empty())
            .or_else(sys_locale::get_locale)
            .unwrap_or_default();
        let mut result = Self {
            language: data
                .languages
                .iter()
                .position(|l| locale.starts_with(l["code"].as_str().unwrap()))
                .unwrap_or(0),
            ..Self::default()
        };
        if let Some(saved) = preferences_path()
            .and_then(|p| std::fs::read(p).ok())
            .and_then(|b| serde_json::from_slice::<SavedPreferences>(&b).ok())
        {
            result.language = data
                .languages
                .iter()
                .position(|l| l["code"] == saved.language)
                .unwrap_or(result.language);
            result.theme = data
                .themes
                .iter()
                .position(|t| t["id"] == saved.theme)
                .unwrap_or(0);
            result.accent = ACCENTS.iter().position(|a| *a == saved.accent).unwrap_or(0);
        }
        result
    }
    pub fn save(&self, data: &Data) -> Result<(), Box<dyn std::error::Error>> {
        let path = preferences_path().ok_or("répertoire utilisateur introuvable")?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let bytes = serde_json::to_vec_pretty(&SavedPreferences {
            language: data.languages[self.language]["code"]
                .as_str()
                .unwrap()
                .into(),
            theme: data.themes[self.theme]["id"].as_str().unwrap().into(),
            accent: ACCENTS[self.accent].into(),
        })?;
        let temp = path.with_extension(format!("{}.tmp", std::process::id()));
        std::fs::write(&temp, bytes)?;
        #[cfg(target_os = "windows")]
        if path.exists() {
            let _ = std::fs::remove_file(&path);
        }
        std::fs::rename(temp, path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_key_layers_and_translations_are_complete() {
        let data = Data::load();
        assert_eq!(data.languages.len(), 6);
        assert_eq!(data.themes.len(), 8);
        for mode in [Mode::Standard, Mode::Scientifique] {
            for second in [false, true] {
                for hyp in [false, true] {
                    let rows = data.rows(mode, second, hyp);
                    assert_eq!(rows.len(), if mode == Mode::Standard { 6 } else { 8 });
                    for key in rows.into_iter().flatten() {
                        for language in &data.languages {
                            assert!(
                                language["touches"][&key.id].is_string(),
                                "{} {}",
                                language["code"],
                                key.id
                            );
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn formatting_preserves_both_decimal_and_grouping_separators() {
        let data = Data::load();
        assert_eq!(
            format("1\u{202f}234,56 + 2", &data.languages[1]),
            "1,234.56 + 2"
        );
        assert_eq!(format("1\u{202f}234,56", &data.languages[2]), "1.234,56");
        assert_eq!(superscript("sin{−1}"), "sin⁻¹");
    }
    #[test]
    fn keyboard_respects_modifiers_and_numlock_independent_physical_keys() {
        let data = Data::load();
        let end = NativeKey::Named(NamedKey::End);
        let keypad = PhysicalKey::Code(KeyCode::Numpad1);
        assert_eq!(
            keyboard_action(&end, keypad, ModifiersState::empty(), &data.shortcuts).as_deref(),
            Some("d1")
        );
        assert_eq!(
            keyboard_action(&end, keypad, ModifiersState::CONTROL, &data.shortcuts),
            None
        );
        let c = NativeKey::Character("c".into());
        assert_eq!(
            keyboard_action(
                &c,
                PhysicalKey::Code(KeyCode::KeyC),
                ModifiersState::CONTROL,
                &data.shortcuts
            )
            .as_deref(),
            Some("copy")
        );
        assert_eq!(
            keyboard_action(
                &NativeKey::Character("é".into()),
                PhysicalKey::Code(KeyCode::Digit2),
                ModifiersState::ALT,
                &data.shortcuts
            )
            .as_deref(),
            Some("scientifique")
        );
    }
    #[test]
    fn native_controller_keeps_scientific_layers_and_decimal_engine() {
        let data = Data::load();
        let mut engine = moteur::Engine::new();
        for id in ["d0", "dec", "d1", "add", "d0", "dec", "d2", "eq"] {
            engine.press(
                data.find_key(id, Mode::Standard, false, false)
                    .unwrap()
                    .key
                    .unwrap(),
            );
        }
        assert_eq!(engine.snapshot().display, "0,3");
        assert_eq!(
            data.find_key("asin", Mode::Scientifique, true, false)
                .unwrap()
                .key,
            Some(Key::Unary(moteur::Unary::Asin))
        );
        assert_eq!(
            data.find_key("asinh", Mode::Scientifique, true, true)
                .unwrap()
                .key,
            Some(Key::Unary(moteur::Unary::Asinh))
        );
        assert_eq!(numpad(KeyCode::Numpad1), Some("d1"));
        assert_eq!(numpad(KeyCode::NumpadDecimal), Some("dec"));
        assert_eq!(
            memory_key("memoryRecall", -1),
            Some(Key::MemoryRecall(None))
        );
    }
    #[test]
    fn preferences_path_and_serialization() {
        let data = Data::load();
        let prefs = Preferences {
            language: 1,
            theme: 2,
            accent: 3,
        };
        assert!(preferences_path().is_some());
        let json = serde_json::to_string(&SavedPreferences {
            language: data.languages[prefs.language]["code"]
                .as_str()
                .unwrap()
                .into(),
            theme: data.themes[prefs.theme]["id"].as_str().unwrap().into(),
            accent: ACCENTS[prefs.accent].into(),
        })
        .unwrap();
        let deserialized: SavedPreferences = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.language, "en");
        assert_eq!(deserialized.theme, "frappe");
        assert_eq!(deserialized.accent, "lavender");
    }
    #[test]
    fn cli_options_parsing() {
        let data = Data::load();
        let args = vec![
            "calculatrice".into(),
            "--lang".into(),
            "en".into(),
            "--theme".into(),
            "latte".into(),
            "--accent".into(),
            "peach".into(),
            "--mode".into(),
            "scientifique".into(),
        ];
        let opts = CliOptions::parse(&args, &data).unwrap();
        assert_eq!(opts.language, Some(1));
        assert_eq!(opts.theme, Some(3));
        assert_eq!(opts.accent, Some(4));
        assert_eq!(opts.mode, Some(Mode::Scientifique));

        let short_args = vec![
            "calculatrice".into(),
            "-l".into(),
            "de".into(),
            "-t".into(),
            "nord".into(),
            "-m".into(),
            "std".into(),
        ];
        let short_opts = CliOptions::parse(&short_args, &data).unwrap();
        assert_eq!(short_opts.language, Some(2));
        assert_eq!(short_opts.theme, Some(6));
        assert_eq!(short_opts.mode, Some(Mode::Standard));

        let help_args = vec!["calculatrice".into(), "-h".into()];
        assert!(CliOptions::parse(&help_args, &data).unwrap().help);

        let err_args = vec!["calculatrice".into(), "--theme".into(), "inexistant".into()];
        assert!(CliOptions::parse(&err_args, &data).is_err());
    }
}
