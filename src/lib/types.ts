// Types partagés avec le moteur Rust (voir src-tauri/moteur/src/lib.rs).

export type Mode = 'standard' | 'scientifique';
export type Angle = 'deg' | 'rad' | 'grad';
export type CalcErrorKind = 'divZero' | 'undefined' | 'invalid' | 'overflow';
export type BinOp = 'add' | 'sub' | 'mul' | 'div' | 'mod' | 'pow' | 'root';
export type Unary =
  | 'negate' | 'square' | 'cube' | 'sqrt' | 'cbrt' | 'inverse' | 'abs' | 'fact'
  | 'pow10' | 'pow2' | 'exp' | 'log' | 'ln'
  | 'sin' | 'cos' | 'tan' | 'asin' | 'acos' | 'atan'
  | 'sinh' | 'cosh' | 'tanh' | 'asinh' | 'acosh' | 'atanh';

export type Key =
  | { type: 'digit'; value: number }
  | { type: 'op'; value: BinOp }
  | { type: 'unary'; value: Unary }
  | { type: 'constant'; value: 'pi' | 'e' }
  | { type: 'decimal' | 'percent' | 'equals' | 'clear' | 'clearEntry' | 'backspace' | 'open' | 'close' }
  | { type: 'memoryStore' | 'historyClear' }
  | { type: 'memoryRecall' | 'memoryAdd' | 'memorySub' | 'memoryClear'; value: number | null }
  | { type: 'historyRecall'; value: number }
  | { type: 'setMode'; value: Mode }
  | { type: 'setAngle'; value: Angle }
  | { type: 'paste'; value: string };

export interface HistoryItem {
  expression: string;
  result: string;
}

export interface Snapshot {
  display: string;
  expression: string;
  error: boolean;
  errorKind: CalcErrorKind | null;
  mode: Mode;
  angle: Angle;
  openParens: number;
  history: HistoryItem[];
  memory: string[];
}
