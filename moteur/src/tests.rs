use super::*;

/// Tape une suite de touches : chiffres, `.` `+` `-` `*` `/` `^` `%` `(` `)` `=`.
fn tape(e: &mut Engine, keys: &str) -> Snapshot {
    for c in keys.chars() {
        let key = match c {
            '0'..='9' => Key::Digit(c as u8 - b'0'),
            '.' => Key::Decimal,
            '+' => Key::Op(BinOp::Add),
            '-' => Key::Op(BinOp::Sub),
            '*' => Key::Op(BinOp::Mul),
            '/' => Key::Op(BinOp::Div),
            '^' => Key::Op(BinOp::Pow),
            '%' => Key::Percent,
            '(' => Key::Open,
            ')' => Key::Close,
            '=' => Key::Equals,
            ' ' => continue,
            _ => panic!("touche inconnue {c}"),
        };
        e.press(key);
    }
    e.snapshot()
}

fn ecran(e: &mut Engine, keys: &str) -> String {
    tape(e, keys).display.replace(NNBSP, " ")
}

fn sci() -> Engine {
    let mut e = Engine::new();
    e.press(Key::SetMode(Mode::Scientifique));
    e
}

#[test]
fn decimal_exact() {
    assert_eq!(ecran(&mut Engine::new(), "0.1+0.2="), "0,3");
    assert_eq!(ecran(&mut Engine::new(), "1/3*3="), "1");
}

#[test]
fn standard_execution_immediate() {
    let mut e = Engine::new();
    assert_eq!(ecran(&mut e, "2+3*"), "5");
    assert_eq!(ecran(&mut e, "4="), "20");
    assert_eq!(e.snapshot().expression, "2 + 3 × 4 =");
}

#[test]
fn scientifique_priorite_et_parentheses() {
    assert_eq!(ecran(&mut sci(), "2+3*4="), "14");
    assert_eq!(ecran(&mut sci(), "(2+3)*4="), "20");
    assert_eq!(ecran(&mut sci(), "2^3^2="), "512");
    let mut e = sci();
    assert_eq!(ecran(&mut e, "2*3+"), "6");
    assert_eq!(ecran(&mut e, "(1+2"), "2");
    assert_eq!(e.snapshot().open_parens, 1);
    assert_eq!(ecran(&mut e, "="), "9");
    assert_eq!(e.snapshot().expression, "2 × 3 + (1 + 2) =");
}

#[test]
fn egal_repete() {
    assert_eq!(ecran(&mut Engine::new(), "5+3=="), "11");
    assert_eq!(ecran(&mut Engine::new(), "5+="), "10");
}

#[test]
fn changer_operateur() {
    assert_eq!(ecran(&mut Engine::new(), "5+*3="), "15");
}

#[test]
fn pourcentage() {
    assert_eq!(ecran(&mut Engine::new(), "200+10%="), "220");
    assert_eq!(ecran(&mut Engine::new(), "50*10%="), "5");
}

#[test]
fn erreurs() {
    let mut e = Engine::new();
    let s = tape(&mut e, "5/0=");
    assert!(s.error);
    assert_eq!(s.display, "Division par zéro impossible");
    assert_eq!(ecran(&mut e, "7"), "7");
    assert_eq!(
        tape(&mut Engine::new(), "0/0=").display,
        "Résultat non défini"
    );
    let mut e = Engine::new();
    tape(&mut e, "9-");
    e.press(Key::Digit(4));
    e.press(Key::Unary(Unary::Negate));
    let s = e.press(Key::Unary(Unary::Sqrt));
    assert_eq!(s.display, "Entrée non valide");
}

#[test]
fn fonctions_et_libelles() {
    let mut e = Engine::new();
    tape(&mut e, "5+9");
    let s = e.press(Key::Unary(Unary::Sqrt));
    assert_eq!(s.display, "3");
    assert_eq!(s.expression, "5 + √(9)");
    assert_eq!(ecran(&mut e, "="), "8");

    let mut e = Engine::new();
    tape(&mut e, "4");
    e.press(Key::Unary(Unary::Square));
    let s = e.press(Key::Unary(Unary::Inverse));
    assert_eq!(s.expression, "1/(4²)");
    assert_eq!(s.display, "0,0625");
}

#[test]
fn fonction_sur_parenthese() {
    let mut e = sci();
    tape(&mut e, "(2+3)");
    let s = e.press(Key::Unary(Unary::Square));
    assert_eq!(s.expression, "(2 + 3)²");
    assert_eq!(ecran(&mut e, "+1="), "26");
}

#[test]
fn trigonometrie() {
    let mut e = sci();
    let sin = |e: &mut Engine, keys: &str, u: Unary| {
        e.press(Key::Clear);
        tape(e, keys);
        e.press(Key::Unary(u)).display
    };
    assert_eq!(sin(&mut e, "30", Unary::Sin), "0,5");
    assert_eq!(sin(&mut e, "180", Unary::Sin), "0");
    assert_eq!(sin(&mut e, "60", Unary::Cos), "0,5");
    assert_eq!(sin(&mut e, "45", Unary::Tan), "1");
    assert_eq!(sin(&mut e, "90", Unary::Tan), "Entrée non valide");
    assert_eq!(sin(&mut e, "1", Unary::Atan), "45");
    e.press(Key::SetAngle(Angle::Rad));
    e.press(Key::Clear);
    e.press(Key::Constant(Constant::Pi));
    assert_eq!(e.press(Key::Unary(Unary::Cos)).display, "-1");
}

#[test]
fn puissances_et_racines() {
    assert_eq!(ecran(&mut sci(), "2^10="), "1 024");
    let mut e = sci();
    tape(&mut e, "27");
    e.press(Key::Op(BinOp::Root));
    assert_eq!(ecran(&mut e, "3="), "3");
    let mut e = sci();
    tape(&mut e, "8");
    e.press(Key::Unary(Unary::Negate));
    assert_eq!(e.press(Key::Unary(Unary::Cbrt)).display, "-2");
    let mut e = sci();
    tape(&mut e, "5");
    assert_eq!(e.press(Key::Unary(Unary::Fact)).display, "120");
    let mut e = sci();
    tape(&mut e, "1000");
    assert_eq!(e.press(Key::Unary(Unary::Log)).display, "3");
}

#[test]
fn mise_en_forme_francaise() {
    assert_eq!(ecran(&mut Engine::new(), "1234567.50"), "1 234 567,50");
    assert_eq!(ecran(&mut Engine::new(), "1234567.5*2="), "2 469 135");
    assert_eq!(
        ecran(&mut Engine::new(), "99999999*99999999="),
        "9 999 999 800 000 001"
    );
    assert_eq!(
        ecran(&mut Engine::new(), "99999999*99999999*1000="),
        "9,999999800000001e+18"
    );
    assert_eq!(ecran(&mut Engine::new(), "1/3="), "0,3333333333333333");
}

#[test]
fn saisie_limitee_et_correction() {
    let mut e = Engine::new();
    tape(&mut e, "12345678901234567890");
    assert_eq!(e.snapshot().display.replace(NNBSP, ""), "1234567890123456");
    let mut e = Engine::new();
    tape(&mut e, "123");
    e.press(Key::Backspace);
    assert_eq!(e.snapshot().display, "12");
    e.press(Key::Backspace);
    e.press(Key::Backspace);
    assert_eq!(e.snapshot().display, "0");
}

#[test]
fn memoire() {
    let mut e = Engine::new();
    tape(&mut e, "5");
    e.press(Key::MemoryStore);
    tape(&mut e, "+3=");
    e.press(Key::MemoryAdd(None));
    assert_eq!(e.snapshot().memory, vec!["13"]);
    e.press(Key::Clear);
    assert_eq!(e.press(Key::MemoryRecall(None)).display, "13");
    e.press(Key::MemoryClear(None));
    assert!(e.snapshot().memory.is_empty());
}

#[test]
fn historique() {
    let mut e = Engine::new();
    tape(&mut e, "2+2=");
    tape(&mut e, "3*3=");
    let s = e.snapshot();
    assert_eq!(s.history.len(), 2);
    assert_eq!(s.history[0].expression, "3 × 3 =");
    let s = e.press(Key::HistoryRecall(1));
    assert_eq!(s.display, "4");
    assert_eq!(s.expression, "2 + 2 =");
}

#[test]
fn coller() {
    let mut e = Engine::new();
    assert_eq!(
        e.press(Key::Paste("1 234,5".into()))
            .display
            .replace(NNBSP, " "),
        "1 234,5"
    );
    assert_eq!(
        e.press(Key::Paste("1,234.5".into()))
            .display
            .replace(NNBSP, " "),
        "1 234,5"
    );
    assert_eq!(e.press(Key::Paste("−12".into())).display, "-12");
    assert_eq!(e.press(Key::Paste("bonjour".into())).display, "-12");
}

#[test]
fn touches_json() {
    let k: Key = serde_json::from_str(r#"{"type":"digit","value":7}"#).unwrap();
    assert_eq!(k, Key::Digit(7));
    let k: Key = serde_json::from_str(r#"{"type":"clearEntry"}"#).unwrap();
    assert_eq!(k, Key::ClearEntry);
    let k: Key = serde_json::from_str(r#"{"type":"memoryRecall","value":null}"#).unwrap();
    assert_eq!(k, Key::MemoryRecall(None));
    let k: Key = serde_json::from_str(r#"{"type":"setMode","value":"scientifique"}"#).unwrap();
    assert_eq!(k, Key::SetMode(Mode::Scientifique));
}
