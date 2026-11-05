import type { BinOp, Key, Unary } from './types';

export type Genre = 'chiffre' | 'fonction' | 'egal';

export interface Touche {
  id: string;
  /** Texte de la touche ; `{…}` marque un exposant : `x{2}`. */
  label: string;
  /** Nom lu par les lecteurs d'écran. */
  aria: string;
  genre: Genre;
  /** Action envoyée au moteur ; absente pour les bascules 2nde / hyp. */
  key?: Key;
  bascule?: 'seconde' | 'hyp';
  /** Reste utilisable quand une erreur est affichée. */
  sure?: boolean;
}

/** Une case du pavé, avec ses variantes 2nde et hyp éventuelles. */
export interface Case {
  base: Touche;
  seconde?: Touche;
  hyp?: Touche;
  hypSeconde?: Touche;
}

const NOMS = ['Zéro', 'Un', 'Deux', 'Trois', 'Quatre', 'Cinq', 'Six', 'Sept', 'Huit', 'Neuf'];

const chiffre = (n: number): Touche => ({
  id: `d${n}`,
  label: String(n),
  aria: NOMS[n],
  genre: 'chiffre',
  key: { type: 'digit', value: n },
  sure: true,
});
const op = (id: string, label: string, aria: string, value: BinOp): Touche => ({
  id, label, aria, genre: 'fonction', key: { type: 'op', value },
});
const un = (id: string, label: string, aria: string, value: Unary): Touche => ({
  id, label, aria, genre: 'fonction', key: { type: 'unary', value },
});

const T = {
  pct: { id: 'pct', label: '%', aria: 'Pourcentage', genre: 'fonction', key: { type: 'percent' } },
  ce: { id: 'ce', label: 'CE', aria: "Effacer l'entrée", genre: 'fonction', key: { type: 'clearEntry' }, sure: true },
  c: { id: 'clear', label: 'C', aria: 'Tout effacer', genre: 'fonction', key: { type: 'clear' }, sure: true },
  back: { id: 'back', label: '⌫', aria: 'Retour arrière', genre: 'fonction', key: { type: 'backspace' }, sure: true },
  eq: { id: 'eq', label: '=', aria: 'Égal', genre: 'egal', key: { type: 'equals' }, sure: true },
  dec: { id: 'dec', label: ',', aria: 'Virgule', genre: 'chiffre', key: { type: 'decimal' }, sure: true },
  neg: { ...un('neg', '+/−', 'Changer de signe', 'negate'), genre: 'chiffre' },
  open: { id: 'open', label: '(', aria: 'Parenthèse ouvrante', genre: 'fonction', key: { type: 'open' } },
  close: { id: 'close', label: ')', aria: 'Parenthèse fermante', genre: 'fonction', key: { type: 'close' } },
  pi: { id: 'pi', label: 'π', aria: 'Pi', genre: 'fonction', key: { type: 'constant', value: 'pi' }, sure: true },
  e: { id: 'e', label: 'e', aria: "Nombre d'Euler", genre: 'fonction', key: { type: 'constant', value: 'e' }, sure: true },
  seconde: { id: 'seconde', label: '2nde', aria: 'Fonctions secondaires', genre: 'fonction', bascule: 'seconde' },
  hyp: { id: 'hyp', label: 'hyp', aria: 'Fonctions hyperboliques', genre: 'fonction', bascule: 'hyp' },
  div: op('div', '÷', 'Diviser par', 'div'),
  mul: op('mul', '×', 'Multiplier par', 'mul'),
  sub: op('sub', '−', 'Moins', 'sub'),
  add: op('add', '+', 'Plus', 'add'),
  mod: op('mod', 'mod', 'Modulo', 'mod'),
  pow: op('pow', 'x{y}', 'Puissance', 'pow'),
  root: op('root', '{y}√x', 'Racine y-ième', 'root'),
  inv: un('inv', '1/x', 'Inverse', 'inverse'),
  sqr: un('sqr', 'x{2}', 'Carré', 'square'),
  cube: un('cube', 'x{3}', 'Cube', 'cube'),
  sqrt: un('sqrt', '√x', 'Racine carrée', 'sqrt'),
  cbrt: un('cbrt', '{3}√x', 'Racine cubique', 'cbrt'),
  abs: un('abs', '|x|', 'Valeur absolue', 'abs'),
  fact: un('fact', 'n!', 'Factorielle', 'fact'),
  exp: un('exp', 'e{x}', 'Exponentielle', 'exp'),
  pow10: un('pow10', '10{x}', 'Dix puissance x', 'pow10'),
  pow2: un('pow2', '2{x}', 'Deux puissance x', 'pow2'),
  log: un('log', 'log', 'Logarithme décimal', 'log'),
  ln: un('ln', 'ln', 'Logarithme népérien', 'ln'),
} satisfies Record<string, Touche>;

const trigo = (nom: 'sin' | 'cos' | 'tan', aria: string): Case => ({
  base: un(nom, nom, aria, nom),
  seconde: un(`a${nom}`, `${nom}{−1}`, `Arc ${aria.toLowerCase()}`, `a${nom}`),
  hyp: un(`${nom}h`, `${nom}h`, `${aria} hyperbolique`, `${nom}h`),
  hypSeconde: un(`a${nom}h`, `${nom}h{−1}`, `Argument ${aria.toLowerCase()} hyperbolique`, `a${nom}h`),
});

const c = (base: Touche, seconde?: Touche): Case => ({ base, seconde });
const ligne = (...touches: (Touche | Case)[]): Case[] =>
  touches.map((t) => ('base' in t ? t : c(t)));
const d = chiffre;

export const STANDARD: Case[][] = [
  ligne(T.pct, T.ce, T.c, T.back),
  ligne(T.inv, T.sqr, T.sqrt, T.div),
  ligne(d(7), d(8), d(9), T.mul),
  ligne(d(4), d(5), d(6), T.sub),
  ligne(d(1), d(2), d(3), T.add),
  ligne(T.neg, d(0), T.dec, T.eq),
];

export const SCIENTIFIQUE: Case[][] = [
  ligne(T.seconde, T.hyp, trigo('sin', 'Sinus'), trigo('cos', 'Cosinus'), trigo('tan', 'Tangente')),
  ligne(T.pi, T.e, T.ce, T.c, T.back),
  ligne(c(T.sqr, T.cube), T.inv, T.abs, T.exp, T.mod),
  ligne(c(T.sqrt, T.cbrt), T.open, T.close, T.fact, T.div),
  ligne(c(T.pow, T.root), d(7), d(8), d(9), T.mul),
  ligne(c(T.pow10, T.pow2), d(4), d(5), d(6), T.sub),
  ligne(T.log, d(1), d(2), d(3), T.add),
  ligne(T.ln, T.neg, d(0), T.dec, T.eq),
];

export function resoudre(c: Case, seconde: boolean, hyp: boolean): Touche {
  if (hyp && seconde && c.hypSeconde) return c.hypSeconde;
  if (hyp && c.hyp) return c.hyp;
  if (seconde && c.seconde) return c.seconde;
  return c.base;
}

/**
 * Correspondance physique du pavé numérique (`KeyboardEvent.code`).
 * Fonctionne sous Wayland/WebKitGTK même lorsque Verr Num (NumLock) est désactivé
 * ou que le serveur d'affichage renvoie des clés XKB `KP_*` / `Insert` / `Delete`.
 */
export const PAVENUM_CODES: Record<string, string> = {
  Numpad0: 'd0',
  Numpad1: 'd1',
  Numpad2: 'd2',
  Numpad3: 'd3',
  Numpad4: 'd4',
  Numpad5: 'd5',
  Numpad6: 'd6',
  Numpad7: 'd7',
  Numpad8: 'd8',
  Numpad9: 'd9',
  NumpadDecimal: 'dec',
  NumpadComma: 'dec',
  NumpadAdd: 'add',
  NumpadSubtract: 'sub',
  NumpadMultiply: 'mul',
  NumpadDivide: 'div',
  NumpadEnter: 'eq',
  NumpadEqual: 'eq',
  NumpadParenLeft: 'open',
  NumpadParenRight: 'close',
  NumpadBackspace: 'back',
  NumpadClear: 'ce',
};

/** Raccourcis clavier par `KeyboardEvent.key` (y compris variantes XKB/Wayland KP_*). */
export const RACCOURCIS: Record<string, string> = {
  ...Object.fromEntries([...'0123456789'].map((n) => [n, `d${n}`])),
  ...Object.fromEntries([...'0123456789'].map((n) => [`KP_${n}`, `d${n}`])),
  '.': 'dec',
  ',': 'dec',
  Decimal: 'dec',
  Separator: 'dec',
  KP_Decimal: 'dec',
  KP_Separator: 'dec',
  KP_Delete: 'dec',
  '+': 'add',
  Add: 'add',
  KP_Add: 'add',
  '-': 'sub',
  '−': 'sub',
  Subtract: 'sub',
  KP_Subtract: 'sub',
  '*': 'mul',
  '×': 'mul',
  Multiply: 'mul',
  KP_Multiply: 'mul',
  '/': 'div',
  '÷': 'div',
  Divide: 'div',
  KP_Divide: 'div',
  Enter: 'eq',
  KP_Enter: 'eq',
  '=': 'eq',
  KP_Equal: 'eq',
  Escape: 'clear',
  Delete: 'ce',
  Clear: 'ce',
  Backspace: 'back',
  '%': 'pct',
  '(': 'open',
  ')': 'close',
  '^': 'pow',
  '!': 'fact',
  r: 'inv',
  '@': 'sqrt',
  q: 'sqr',
  F9: 'neg',
  p: 'pi',
  s: 'sin',
  o: 'cos',
  t: 'tan',
  l: 'log',
  n: 'ln',
};

/** Résout un événement clavier en identifiant de touche de la calculatrice. */
export function resoudreRaccourciClavier(e: KeyboardEvent): string | null {
  // 1. Pavé numérique via `e.code` (indépendant de Verr Num sous Wayland)
  if (e.code && PAVENUM_CODES[e.code]) {
    return PAVENUM_CODES[e.code];
  }
  // 2. Pavé numérique identifié par `location === 3` (DOM_KEY_LOCATION_NUMPAD)
  if (e.location === 3) {
    const numpadParKey: Record<string, string> = {
      Insert: 'd0',
      End: 'd1',
      ArrowDown: 'd2',
      PageDown: 'd3',
      ArrowLeft: 'd4',
      Clear: 'd5',
      Unidentified: 'd5',
      Begin: 'd5',
      ArrowRight: 'd6',
      Home: 'd7',
      ArrowUp: 'd8',
      PageUp: 'd9',
      Delete: 'dec',
    };
    if (numpadParKey[e.key]) return numpadParKey[e.key];
  }
  // 3. Touche standard via `e.key`
  return RACCOURCIS[e.key] ?? null;
}
