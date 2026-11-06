//! Moteur de la Calculatrice : saisie, priorité des opérations, mémoire et historique.
//!
//! Les quatre opérations, les puissances entières et la racine carrée sont calculées en
//! décimal exact (`rust_decimal`, 28 chiffres) : 0,1 + 0,2 donne bien 0,3.
//! Les fonctions transcendantes (trigonométrie, log, puissances non entières) passent
//! par `f64`, arrondies à 15 chiffres significatifs pour éviter les artefacts binaires.

use std::str::FromStr;

use rust_decimal::prelude::*;
use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Serialize};

/// Nombre maximal de chiffres saisis au clavier (comme la calculatrice de Windows).
const MAX_DIGITS: usize = 16;
/// Chiffres significatifs affichés.
const DISPLAY_DIGITS: u32 = 16;
/// Espace fine insécable : séparateur des milliers en français.
pub const NNBSP: char = '\u{202F}';

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Exécution immédiate : 2 + 3 × 4 = 20.
    #[default]
    Standard,
    /// Priorité des opérations et parenthèses : 2 + 3 × 4 = 14.
    Scientifique,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Angle {
    #[default]
    Deg,
    Rad,
    Grad,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Root,
}

impl BinOp {
    fn symbol(self) -> &'static str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "−",
            BinOp::Mul => "×",
            BinOp::Div => "÷",
            BinOp::Mod => "mod",
            BinOp::Pow => "^",
            BinOp::Root => "yroot",
        }
    }

    fn precedence(self, mode: Mode) -> u8 {
        if mode == Mode::Standard {
            return 1;
        }
        match self {
            BinOp::Add | BinOp::Sub => 1,
            BinOp::Mul | BinOp::Div | BinOp::Mod => 2,
            BinOp::Pow | BinOp::Root => 3,
        }
    }

    fn right_assoc(self, mode: Mode) -> bool {
        mode == Mode::Scientifique && matches!(self, BinOp::Pow | BinOp::Root)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Unary {
    Negate,
    Square,
    Cube,
    Sqrt,
    Cbrt,
    Inverse,
    Abs,
    Fact,
    Pow10,
    Pow2,
    Exp,
    Log,
    Ln,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
}

impl Unary {
    /// Libellé affiché dans la ligne d'expression, par ex. `√(9)` ou `5²`.
    fn label(self, x: &str) -> String {
        let func = |name: &str| {
            if is_grouped(x) {
                format!("{name}{x}")
            } else {
                format!("{name}({x})")
            }
        };
        let postfix = |suffix: &str| {
            if is_simple(x) || is_grouped(x) {
                format!("{x}{suffix}")
            } else {
                format!("({x}){suffix}")
            }
        };
        match self {
            Unary::Negate => func("−"),
            Unary::Square => postfix("²"),
            Unary::Cube => postfix("³"),
            Unary::Sqrt => func("√"),
            Unary::Cbrt => func("∛"),
            Unary::Inverse => func("1/"),
            Unary::Abs => format!("|{x}|"),
            Unary::Fact => postfix("!"),
            Unary::Pow10 => func("10^"),
            Unary::Pow2 => func("2^"),
            Unary::Exp => func("e^"),
            Unary::Log => func("log"),
            Unary::Ln => func("ln"),
            Unary::Sin => func("sin"),
            Unary::Cos => func("cos"),
            Unary::Tan => func("tan"),
            Unary::Asin => func("sin⁻¹"),
            Unary::Acos => func("cos⁻¹"),
            Unary::Atan => func("tan⁻¹"),
            Unary::Sinh => func("sinh"),
            Unary::Cosh => func("cosh"),
            Unary::Tanh => func("tanh"),
            Unary::Asinh => func("sinh⁻¹"),
            Unary::Acosh => func("cosh⁻¹"),
            Unary::Atanh => func("tanh⁻¹"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Constant {
    Pi,
    E,
}

/// Une action de l'utilisateur (bouton, raccourci clavier, panneau…).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum Key {
    Digit(u8),
    Decimal,
    Op(BinOp),
    Unary(Unary),
    Percent,
    Equals,
    Clear,
    ClearEntry,
    Backspace,
    Open,
    Close,
    Constant(Constant),
    MemoryStore,
    MemoryRecall(Option<usize>),
    MemoryAdd(Option<usize>),
    MemorySub(Option<usize>),
    MemoryClear(Option<usize>),
    HistoryRecall(usize),
    HistoryClear,
    SetMode(Mode),
    SetAngle(Angle),
    Paste(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CalcError {
    DivZero,
    Undefined,
    Invalid,
    Overflow,
}

impl CalcError {
    pub fn message(self) -> &'static str {
        match self {
            CalcError::DivZero => "Division par zéro impossible",
            CalcError::Undefined => "Résultat non défini",
            CalcError::Invalid => "Entrée non valide",
            CalcError::Overflow => "Dépassement de capacité",
        }
    }
}

type R<T> = Result<T, CalcError>;

#[derive(Clone, Debug, PartialEq)]
enum Tok {
    Num(Decimal, String),
    Op(BinOp),
    Open,
    Close,
}

#[derive(Clone, Debug, Serialize)]
pub struct HistoryItem {
    pub expression: String,
    pub result: String,
    #[serde(skip)]
    value: Decimal,
}

/// État complet envoyé à l'interface après chaque action.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub display: String,
    pub expression: String,
    pub error: bool,
    pub error_kind: Option<CalcError>,
    pub mode: Mode,
    pub angle: Angle,
    pub open_parens: usize,
    pub history: Vec<HistoryItem>,
    pub memory: Vec<String>,
}

#[derive(Debug, Default)]
pub struct Engine {
    mode: Mode,
    angle: Angle,
    /// Expression validée (nombres, opérateurs, parenthèses).
    tokens: Vec<Tok>,
    /// Valeur actuellement affichée.
    entry: Decimal,
    /// Texte en cours de saisie, `None` si la valeur vient d'un calcul.
    input: Option<String>,
    /// Libellé de la valeur affichée si elle provient d'une fonction (`√(9)`).
    label: Option<String>,
    /// La valeur affichée est un nouvel opérande, pas un simple résultat intermédiaire.
    fresh: bool,
    after_equals: bool,
    /// Dernière opération, pour répéter `=`.
    last: Option<(BinOp, Decimal)>,
    /// Ligne d'expression figée (après `=` ou une erreur).
    frozen: Option<String>,
    error: Option<CalcError>,
    history: Vec<HistoryItem>,
    memory: Vec<Decimal>,
}

#[derive(Clone, Copy)]
enum Flush {
    /// Réduire tout ce qui est prioritaire sur l'opérateur à venir.
    Op(BinOp),
    /// Ne rien réduire de plus (valeur d'une parenthèse qui vient d'être fermée).
    Nothing,
    /// Tout réduire, en fermant les parenthèses restantes.
    All,
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn press(&mut self, key: Key) -> Snapshot {
        self.apply(key);
        self.snapshot()
    }

    pub fn snapshot(&self) -> Snapshot {
        let display = if let Some(e) = self.error {
            e.message().to_string()
        } else if let Some(s) = &self.input {
            format_input(s)
        } else {
            format_value(self.entry)
        };
        let expression = match &self.frozen {
            Some(f) => f.clone(),
            None => {
                let mut toks = self.tokens.clone();
                if self.fresh {
                    if let Some(l) = &self.label {
                        toks.push(Tok::Num(self.entry, l.clone()));
                    }
                }
                render(&toks)
            }
        };
        Snapshot {
            display,
            expression,
            error: self.error.is_some(),
            error_kind: self.error,
            mode: self.mode,
            angle: self.angle,
            open_parens: self.open_parens(),
            history: self.history.clone(),
            memory: self.memory.iter().map(|m| format_value(*m)).collect(),
        }
    }

    fn apply(&mut self, key: Key) {
        match key {
            Key::Digit(d) => self.digit(d),
            Key::Decimal => self.decimal(),
            Key::Op(o) => self.op(o),
            Key::Unary(u) => self.unary(u),
            Key::Percent => self.percent(),
            Key::Equals => self.equals(),
            Key::Clear => self.reset(),
            Key::ClearEntry => self.clear_entry(),
            Key::Backspace => self.backspace(),
            Key::Open => self.open_paren(),
            Key::Close => self.close_paren(),
            Key::Constant(c) => {
                self.begin_entry();
                let (v, l) = match c {
                    Constant::Pi => (Decimal::PI, "π"),
                    Constant::E => (Decimal::E, "e"),
                };
                self.set_operand(v, Some(l.to_string()));
            }
            Key::MemoryStore => {
                if self.error.is_none() {
                    self.memory.insert(0, self.entry);
                    self.input = None;
                }
            }
            Key::MemoryRecall(i) => {
                if let Some(v) = self.memory.get(i.unwrap_or(0)).copied() {
                    self.begin_entry();
                    self.set_operand(v, None);
                }
            }
            Key::MemoryAdd(i) => self.memory_update(i, false),
            Key::MemorySub(i) => self.memory_update(i, true),
            Key::MemoryClear(None) => self.memory.clear(),
            Key::MemoryClear(Some(i)) => {
                if i < self.memory.len() {
                    self.memory.remove(i);
                }
            }
            Key::HistoryRecall(i) => {
                if let Some(item) = self.history.get(i).cloned() {
                    self.reset();
                    self.entry = item.value;
                    self.frozen = Some(item.expression);
                    self.after_equals = true;
                }
            }
            Key::HistoryClear => self.history.clear(),
            Key::SetMode(m) => {
                if m != self.mode {
                    self.mode = m;
                    self.reset();
                }
            }
            Key::SetAngle(a) => self.angle = a,
            Key::Paste(text) => {
                if let Some(v) = parse_paste(&text) {
                    self.begin_entry();
                    let s = v.normalize().to_string();
                    let digits = s.chars().filter(char::is_ascii_digit).count();
                    self.set_operand(v, None);
                    if digits <= MAX_DIGITS {
                        self.input = Some(s);
                    }
                }
            }
        }
    }

    // ---------- Saisie ----------

    /// Efface le calcul en cours (l'historique, la mémoire et le mode sont conservés).
    fn reset(&mut self) {
        self.tokens.clear();
        self.entry = Decimal::ZERO;
        self.input = None;
        self.label = None;
        self.fresh = false;
        self.after_equals = false;
        self.last = None;
        self.frozen = None;
        self.error = None;
    }

    /// Prépare l'arrivée d'un nouvel opérande (chiffre, constante, rappel mémoire…).
    fn begin_entry(&mut self) {
        if self.error.is_some() {
            self.reset();
        }
        if self.after_equals {
            self.tokens.clear();
            self.after_equals = false;
            self.frozen = None;
            self.last = None;
        }
        if matches!(self.tokens.last(), Some(Tok::Close)) {
            // « (2 + 3) 4 » se lit « (2 + 3) × 4 ».
            self.tokens.push(Tok::Op(BinOp::Mul));
        }
        self.label = None;
    }

    fn set_operand(&mut self, v: Decimal, label: Option<String>) {
        self.entry = v;
        self.label = label;
        self.input = None;
        self.fresh = true;
    }

    fn digit(&mut self, d: u8) {
        if d > 9 {
            return;
        }
        if self.input.is_none() {
            self.begin_entry();
            self.input = Some(String::new());
        }
        let s = self.input.as_mut().expect("saisie en cours");
        let c = char::from(b'0' + d);
        if s == "0" || s == "-0" {
            s.pop();
            s.push(c);
        } else if s.chars().filter(char::is_ascii_digit).count() < MAX_DIGITS {
            s.push(c);
        }
        self.entry = parse_input(s);
        self.fresh = true;
    }

    fn decimal(&mut self) {
        match &mut self.input {
            None => {
                self.begin_entry();
                self.input = Some("0.".into());
                self.entry = Decimal::ZERO;
            }
            Some(s) if !s.contains('.') => {
                if s.is_empty() || s == "-" {
                    s.push('0');
                }
                s.push('.');
            }
            Some(_) => {}
        }
        self.fresh = true;
    }

    fn backspace(&mut self) {
        if self.error.is_some() {
            self.reset();
            return;
        }
        if self.after_equals {
            self.frozen = None;
            return;
        }
        if let Some(s) = &mut self.input {
            s.pop();
            if s.is_empty() || s == "-" {
                *s = "0".into();
            }
            self.entry = parse_input(s);
        }
    }

    fn clear_entry(&mut self) {
        if self.error.is_some() || self.after_equals {
            self.reset();
            return;
        }
        self.set_operand(Decimal::ZERO, None);
    }

    // ---------- Opérations ----------

    fn operand_label(&self) -> String {
        self.label
            .clone()
            .unwrap_or_else(|| format_value(self.entry))
    }

    fn set_error(&mut self, e: CalcError, expression: String) {
        self.reset();
        self.error = Some(e);
        self.frozen = Some(expression);
    }

    fn op(&mut self, o: BinOp) {
        if self.error.is_some() {
            return;
        }
        if self.after_equals {
            self.tokens.clear();
            self.tokens.push(Tok::Num(self.entry, self.operand_label()));
            self.after_equals = false;
            self.frozen = None;
        } else if self.fresh
            || self.tokens.is_empty()
            || matches!(self.tokens.last(), Some(Tok::Open))
        {
            self.tokens.push(Tok::Num(self.entry, self.operand_label()));
        } else if matches!(self.tokens.last(), Some(Tok::Op(_))) {
            // Changer d'avis : « 5 + × » devient « 5 × ».
            self.tokens.pop();
        }
        match reduce(&self.tokens, self.mode, Flush::Op(o)) {
            Ok(v) => {
                self.tokens.push(Tok::Op(o));
                self.entry = v;
                self.input = None;
                self.label = None;
                self.fresh = false;
            }
            Err(e) => {
                let expr = format!("{} {}", render(&self.tokens), o.symbol());
                self.set_error(e, expr);
            }
        }
    }

    fn unary(&mut self, u: Unary) {
        if self.error.is_some() {
            return;
        }
        if u == Unary::Negate {
            if let Some(s) = &mut self.input {
                if parse_input(s).is_zero() && !s.contains('.') {
                    return;
                }
                if let Some(rest) = s.strip_prefix('-') {
                    *s = rest.to_string();
                } else {
                    s.insert(0, '-');
                }
                self.entry = parse_input(s);
                return;
            }
        }
        let mut plain = false;
        let base = if !self.fresh && matches!(self.tokens.last(), Some(Tok::Close)) {
            self.take_trailing_group()
        } else if self.after_equals {
            self.tokens.clear();
            self.after_equals = false;
            self.frozen = None;
            format_value(self.entry)
        } else {
            plain = self.label.is_none();
            self.operand_label()
        };
        match apply_unary(u, self.entry, self.angle) {
            Ok(v) => {
                // Changer le signe d'un nombre simple n'a pas besoin de libellé.
                let label = if u == Unary::Negate && plain {
                    None
                } else {
                    Some(u.label(&base))
                };
                self.set_operand(v, label);
            }
            Err(e) => {
                let mut toks = self.tokens.clone();
                toks.push(Tok::Num(self.entry, u.label(&base)));
                self.set_error(e, render(&toks));
            }
        }
    }

    fn percent(&mut self) {
        if self.error.is_some() {
            return;
        }
        let value = match self.tokens.last() {
            Some(Tok::Op(o @ (BinOp::Add | BinOp::Sub))) if !self.after_equals => {
                // 200 + 10 % = 200 + 20 : le pourcentage porte sur la valeur de gauche.
                let left = &self.tokens[..self.tokens.len() - 1];
                reduce(left, self.mode, Flush::Op(*o))
                    .ok()
                    .and_then(|l| l.checked_mul(self.entry))
                    .map(|v| v / Decimal::ONE_HUNDRED)
            }
            _ => Some(self.entry / Decimal::ONE_HUNDRED),
        };
        let Some(v) = value else {
            let expr = render(&self.tokens);
            return self.set_error(CalcError::Overflow, expr);
        };
        if self.after_equals {
            self.tokens.clear();
            self.after_equals = false;
            self.frozen = None;
        }
        self.set_operand(v, Some(format_value(v)));
    }

    fn equals(&mut self) {
        if self.error.is_some() {
            self.reset();
            return;
        }
        if self.after_equals {
            // Répéter la dernière opération : 5 + 3 = = donne 11.
            let Some((o, v)) = self.last else { return };
            self.tokens = vec![
                Tok::Num(self.entry, format_value(self.entry)),
                Tok::Op(o),
                Tok::Num(v, format_value(v)),
            ];
        } else {
            if self.tokens.is_empty() {
                self.frozen = Some(format!("{} =", self.operand_label()));
                self.after_equals = true;
                self.input = None;
                self.label = None;
                self.fresh = false;
                return;
            }
            if self.fresh || matches!(self.tokens.last(), Some(Tok::Op(_) | Tok::Open)) {
                self.tokens.push(Tok::Num(self.entry, self.operand_label()));
            }
            for _ in 0..self.open_parens() {
                self.tokens.push(Tok::Close);
            }
            self.last = match self.tokens.as_slice() {
                [.., Tok::Op(o), Tok::Num(v, _)] => Some((*o, *v)),
                _ => None,
            };
        }
        let expression = format!("{} =", render(&self.tokens));
        match reduce(&self.tokens, self.mode, Flush::All) {
            Ok(v) => {
                self.history.insert(
                    0,
                    HistoryItem {
                        expression: expression.clone(),
                        result: format_value(v),
                        value: v,
                    },
                );
                self.tokens.clear();
                self.entry = v;
                self.input = None;
                self.label = None;
                self.fresh = false;
                self.after_equals = true;
                self.frozen = Some(expression);
            }
            Err(e) => self.set_error(e, expression),
        }
    }

    fn open_parens(&self) -> usize {
        let mut depth: usize = 0;
        for t in &self.tokens {
            match t {
                Tok::Open => depth += 1,
                Tok::Close => depth = depth.saturating_sub(1),
                _ => {}
            }
        }
        depth
    }

    fn open_paren(&mut self) {
        if self.error.is_some() || self.mode == Mode::Standard {
            return;
        }
        if self.after_equals {
            self.tokens.clear();
            self.after_equals = false;
            self.frozen = None;
        } else if self.fresh && (self.input.is_some() || self.label.is_some()) {
            // « 2 ( » se lit « 2 × ( ».
            self.tokens.push(Tok::Num(self.entry, self.operand_label()));
            self.tokens.push(Tok::Op(BinOp::Mul));
        } else if matches!(self.tokens.last(), Some(Tok::Close)) {
            self.tokens.push(Tok::Op(BinOp::Mul));
        }
        self.tokens.push(Tok::Open);
        self.entry = Decimal::ZERO;
        self.input = None;
        self.label = None;
        self.fresh = false;
    }

    fn close_paren(&mut self) {
        if self.error.is_some() || self.mode == Mode::Standard || self.open_parens() == 0 {
            return;
        }
        if self.fresh || matches!(self.tokens.last(), Some(Tok::Op(_) | Tok::Open)) {
            self.tokens.push(Tok::Num(self.entry, self.operand_label()));
        }
        self.tokens.push(Tok::Close);
        match reduce(&self.tokens, self.mode, Flush::Nothing) {
            Ok(v) => {
                self.entry = v;
                self.input = None;
                self.label = None;
                self.fresh = false;
            }
            Err(e) => {
                let expr = render(&self.tokens);
                self.set_error(e, expr);
            }
        }
    }

    /// Retire la dernière parenthèse complète de l'expression et renvoie son libellé.
    fn take_trailing_group(&mut self) -> String {
        let mut depth = 0usize;
        let mut start = 0;
        for (i, t) in self.tokens.iter().enumerate().rev() {
            match t {
                Tok::Close => depth += 1,
                Tok::Open => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        start = i;
                        break;
                    }
                }
                _ => {}
            }
        }
        let group: Vec<Tok> = self.tokens.drain(start..).collect();
        render(&group)
    }

    fn memory_update(&mut self, i: Option<usize>, subtract: bool) {
        if self.error.is_some() {
            return;
        }
        let delta = if subtract { -self.entry } else { self.entry };
        if self.memory.is_empty() {
            self.memory.push(delta);
        } else if let Some(m) = self.memory.get_mut(i.unwrap_or(0)) {
            if let Some(v) = m.checked_add(delta) {
                *m = v;
            }
        }
        self.input = None;
    }
}

// ---------- Évaluation ----------

fn reduce(tokens: &[Tok], mode: Mode, flush: Flush) -> R<Decimal> {
    let mut vals: Vec<Decimal> = Vec::new();
    // `None` représente une parenthèse ouvrante.
    let mut ops: Vec<Option<BinOp>> = Vec::new();

    fn collapse(
        vals: &mut Vec<Decimal>,
        ops: &mut Vec<Option<BinOp>>,
        mode: Mode,
        next: Option<BinOp>,
    ) -> R<()> {
        while let Some(Some(top)) = ops.last().copied() {
            if let Some(o) = next {
                let (pt, po) = (top.precedence(mode), o.precedence(mode));
                if pt < po || (pt == po && o.right_assoc(mode)) {
                    break;
                }
            }
            ops.pop();
            let b = vals.pop().ok_or(CalcError::Invalid)?;
            let a = vals.pop().ok_or(CalcError::Invalid)?;
            vals.push(binary(top, a, b)?);
        }
        Ok(())
    }

    for t in tokens {
        match t {
            Tok::Num(v, _) => vals.push(*v),
            Tok::Open => ops.push(None),
            Tok::Close => {
                collapse(&mut vals, &mut ops, mode, None)?;
                if matches!(ops.last(), Some(None)) {
                    ops.pop();
                }
            }
            Tok::Op(o) => {
                collapse(&mut vals, &mut ops, mode, Some(*o))?;
                ops.push(Some(*o));
            }
        }
    }
    match flush {
        Flush::Op(o) => collapse(&mut vals, &mut ops, mode, Some(o))?,
        Flush::Nothing => {}
        Flush::All => loop {
            collapse(&mut vals, &mut ops, mode, None)?;
            if ops.pop().is_none() {
                break;
            }
        },
    }
    vals.last().copied().ok_or(CalcError::Invalid)
}

fn binary(o: BinOp, a: Decimal, b: Decimal) -> R<Decimal> {
    let checked = |r: Option<Decimal>| r.ok_or(CalcError::Overflow);
    let zero_div = |a: Decimal| {
        if a.is_zero() {
            CalcError::Undefined
        } else {
            CalcError::DivZero
        }
    };
    match o {
        BinOp::Add => checked(a.checked_add(b)),
        BinOp::Sub => checked(a.checked_sub(b)),
        BinOp::Mul => checked(a.checked_mul(b)),
        BinOp::Div if b.is_zero() => Err(zero_div(a)),
        BinOp::Div => checked(a.checked_div(b)),
        BinOp::Mod if b.is_zero() => Err(zero_div(a)),
        BinOp::Mod => checked(a.checked_rem(b)),
        BinOp::Pow => power(a, b),
        BinOp::Root => root(a, b),
    }
}

fn power(a: Decimal, b: Decimal) -> R<Decimal> {
    if b.is_integer() {
        let n = b.to_i64().ok_or(CalcError::Overflow)?;
        if a.is_zero() && n < 0 {
            return Err(CalcError::DivZero);
        }
        return a.checked_powi(n).ok_or(CalcError::Overflow);
    }
    if a.is_sign_negative() && !a.is_zero() {
        return Err(CalcError::Invalid);
    }
    from_f64(f(a).powf(f(b)))
}

/// Racine n-ième de `a`.
fn root(a: Decimal, n: Decimal) -> R<Decimal> {
    if n.is_zero() {
        return Err(CalcError::Invalid);
    }
    if a.is_sign_negative() && !a.is_zero() {
        let odd = n.is_integer() && (n % Decimal::TWO).abs() == Decimal::ONE;
        return if odd {
            root(-a, n).map(|r| -r)
        } else {
            Err(CalcError::Invalid)
        };
    }
    if n == Decimal::TWO {
        return a.sqrt().ok_or(CalcError::Invalid);
    }
    from_f64(f(a).powf(1.0 / f(n)))
}

fn apply_unary(u: Unary, x: Decimal, angle: Angle) -> R<Decimal> {
    let overflow = |r: Option<Decimal>| r.ok_or(CalcError::Overflow);
    let positive = || {
        if x > Decimal::ZERO {
            Ok(())
        } else {
            Err(CalcError::Invalid)
        }
    };
    match u {
        Unary::Negate => Ok(-x),
        Unary::Square => overflow(x.checked_mul(x)),
        Unary::Cube => overflow(x.checked_mul(x).and_then(|s| s.checked_mul(x))),
        Unary::Sqrt => x.sqrt().ok_or(CalcError::Invalid),
        Unary::Cbrt => root(x, Decimal::from(3)),
        Unary::Inverse if x.is_zero() => Err(CalcError::DivZero),
        Unary::Inverse => overflow(Decimal::ONE.checked_div(x)),
        Unary::Abs => Ok(x.abs()),
        Unary::Fact => factorial(x),
        Unary::Pow10 => power(Decimal::TEN, x),
        Unary::Pow2 => power(Decimal::TWO, x),
        Unary::Exp => from_f64(f(x).exp()),
        Unary::Log => positive().and_then(|_| from_f64(f(x).log10())),
        Unary::Ln => positive().and_then(|_| from_f64(f(x).ln())),
        Unary::Sin | Unary::Cos | Unary::Tan => trig(u, x, angle),
        Unary::Asin | Unary::Acos => {
            if x.abs() > Decimal::ONE {
                return Err(CalcError::Invalid);
            }
            let r = if u == Unary::Asin {
                f(x).asin()
            } else {
                f(x).acos()
            };
            from_f64(from_radians(r, angle))
        }
        Unary::Atan => from_f64(from_radians(f(x).atan(), angle)),
        Unary::Sinh => from_f64(f(x).sinh()),
        Unary::Cosh => from_f64(f(x).cosh()),
        Unary::Tanh => from_f64(f(x).tanh()),
        Unary::Asinh => from_f64(f(x).asinh()),
        Unary::Acosh if x < Decimal::ONE => Err(CalcError::Invalid),
        Unary::Acosh => from_f64(f(x).acosh()),
        Unary::Atanh if x.abs() >= Decimal::ONE => Err(CalcError::Invalid),
        Unary::Atanh => from_f64(f(x).atanh()),
    }
}

fn trig(u: Unary, x: Decimal, angle: Angle) -> R<Decimal> {
    // En degrés et en grades, les angles droits donnent des valeurs exactes.
    let quarter = match angle {
        Angle::Deg => Some(Decimal::from(90)),
        Angle::Grad => Some(Decimal::from(100)),
        Angle::Rad => None,
    };
    let reduced = match quarter {
        Some(q) => x % (q * Decimal::from(4)),
        None => x,
    };
    if let Some(q) = quarter {
        if (reduced % q).is_zero() {
            let k = ((reduced / q).to_i64().unwrap_or(0) % 4 + 4) % 4;
            let table: [Option<i64>; 4] = match u {
                Unary::Sin => [Some(0), Some(1), Some(0), Some(-1)],
                Unary::Cos => [Some(1), Some(0), Some(-1), Some(0)],
                _ => [Some(0), None, Some(0), None],
            };
            return table[k as usize]
                .map(Decimal::from)
                .ok_or(CalcError::Invalid);
        }
    }
    let rad = match angle {
        Angle::Deg => f(reduced).to_radians(),
        Angle::Grad => f(reduced) * std::f64::consts::PI / 200.0,
        Angle::Rad => f(x) % std::f64::consts::TAU,
    };
    let r = match u {
        Unary::Sin => rad.sin(),
        Unary::Cos => rad.cos(),
        _ => rad.tan(),
    };
    from_f64(if r.abs() < 1e-15 { 0.0 } else { r })
}

fn from_radians(r: f64, angle: Angle) -> f64 {
    match angle {
        Angle::Deg => r.to_degrees(),
        Angle::Grad => r * 200.0 / std::f64::consts::PI,
        Angle::Rad => r,
    }
}

fn factorial(x: Decimal) -> R<Decimal> {
    if (x.is_sign_negative() && !x.is_zero()) || !x.is_integer() {
        return Err(CalcError::Invalid);
    }
    let n = x.to_u64().ok_or(CalcError::Overflow)?;
    let mut acc = Decimal::ONE;
    for i in 2..=n {
        acc = acc
            .checked_mul(Decimal::from(i))
            .ok_or(CalcError::Overflow)?;
    }
    Ok(acc)
}

fn f(x: Decimal) -> f64 {
    x.to_f64().unwrap_or(0.0)
}

fn from_f64(v: f64) -> R<Decimal> {
    if v.is_nan() {
        return Err(CalcError::Invalid);
    }
    if !v.is_finite() || v.abs() >= 7.9e28 {
        return Err(CalcError::Overflow);
    }
    if v.abs() < 1e-28 {
        return Ok(Decimal::ZERO);
    }
    Decimal::from_f64(v)
        .and_then(|d| d.round_sf(15))
        .map(|d| d.normalize())
        .ok_or(CalcError::Overflow)
}

// ---------- Mise en forme française ----------

fn parse_input(s: &str) -> Decimal {
    let t = s.trim_end_matches('.');
    Decimal::from_str(t).unwrap_or_default()
}

/// Accepte « 1 234,5 », « 1,234.5 », « −12 », « 1e3 »…
fn parse_paste(text: &str) -> Option<Decimal> {
    let mut s: String = text
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace() && *c != NNBSP && *c != '\u{00A0}' && *c != '\'')
        .map(|c| if c == '−' { '-' } else { c })
        .collect();
    let (commas, dots) = (s.matches(',').count(), s.matches('.').count());
    if commas > 0 && dots > 0 {
        let decimal_is_comma = s.rfind(',') > s.rfind('.');
        let (thousands, dec) = if decimal_is_comma {
            (".", ",")
        } else {
            (",", ".")
        };
        s = s.replace(thousands, "").replace(dec, ".");
    } else if commas == 1 {
        s = s.replace(',', ".");
    } else if commas > 1 {
        s = s.replace(',', "");
    } else if dots > 1 {
        s = s.replace('.', "");
    }
    if s.contains(['e', 'E']) {
        Decimal::from_scientific(&s).ok()
    } else {
        Decimal::from_str(&s).ok()
    }
}

fn group_int(int: &str) -> String {
    let mut out = String::new();
    for (i, c) in int.chars().enumerate() {
        if i > 0 && (int.len() - i) % 3 == 0 {
            out.push(NNBSP);
        }
        out.push(c);
    }
    out
}

/// Met en forme un nombre tel qu'il est tapé : « 1234. » devient « 1 234, ».
pub fn format_input(s: &str) -> String {
    let (sign, body) = match s.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", s),
    };
    let body = if body.is_empty() { "0" } else { body };
    match body.split_once('.') {
        Some((int, frac)) => format!("{sign}{},{frac}", group_int(int)),
        None => format!("{sign}{}", group_int(body)),
    }
}

/// Met en forme un résultat : 16 chiffres significatifs, notation scientifique si besoin.
pub fn format_value(d: Decimal) -> String {
    if d.is_zero() {
        return "0".into();
    }
    let r = d
        .round_sf_with_strategy(DISPLAY_DIGITS, RoundingStrategy::MidpointAwayFromZero)
        .unwrap_or(d)
        .normalize();
    if r.is_zero() {
        return "0".into();
    }
    let sign = if r.is_sign_negative() { "-" } else { "" };
    let abs = r.abs().to_string();
    let (int, frac) = abs.split_once('.').unwrap_or((abs.as_str(), ""));
    let exponent: i64 = if int != "0" {
        int.len() as i64 - 1
    } else {
        -(frac.chars().take_while(|c| *c == '0').count() as i64 + 1)
    };
    if (-12..16).contains(&exponent) {
        return if frac.is_empty() {
            format!("{sign}{}", group_int(int))
        } else {
            format!("{sign}{},{frac}", group_int(int))
        };
    }
    let all = format!("{int}{frac}");
    let digits = all.trim_start_matches('0').trim_end_matches('0');
    let (first, rest) = digits.split_at(1);
    let mantissa = if rest.is_empty() {
        first.to_string()
    } else {
        format!("{first},{rest}")
    };
    let exp_sign = if exponent < 0 { '-' } else { '+' };
    format!("{sign}{mantissa}e{exp_sign}{}", exponent.abs())
}

fn is_simple(x: &str) -> bool {
    x.chars()
        .all(|c| c.is_ascii_digit() || c == ',' || c == NNBSP || c == 'π' || c == 'e')
}

/// `(…)` dont la première parenthèse se ferme à la toute fin.
fn is_grouped(x: &str) -> bool {
    if !x.starts_with('(') || !x.ends_with(')') {
        return false;
    }
    let mut depth = 0i32;
    for (i, c) in x.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 && i != x.len() - 1 {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

fn render(tokens: &[Tok]) -> String {
    let mut s = String::new();
    let mut prev_open = false;
    for t in tokens {
        let (piece, space) = match t {
            Tok::Num(_, l) => (l.as_str(), !prev_open),
            Tok::Op(o) => (o.symbol(), true),
            Tok::Open => ("(", !prev_open),
            Tok::Close => (")", false),
        };
        if !s.is_empty() && space {
            s.push(' ');
        }
        s.push_str(piece);
        prev_open = matches!(t, Tok::Open);
    }
    s
}

#[cfg(test)]
mod tests;
