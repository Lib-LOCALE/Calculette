import { invoke } from '@tauri-apps/api/core';
import type { Key, Snapshot } from './types';

const VIDE: Snapshot = {
  display: '0',
  expression: '',
  error: false,
  errorKind: null,
  mode: 'standard',
  angle: 'deg',
  openParens: 0,
  history: [],
  memory: [],
};

/** Pont vers le moteur Rust : chaque touche renvoie l'état complet à afficher. */
class Calculatrice {
  etat = $state<Snapshot>(VIDE);
  // Les touches sont traitées une par une, dans l'ordre de frappe.
  #file: Promise<unknown> = Promise.resolve();

  appuyer(touche: Key): Promise<void> {
    const suite = this.#file.then(async () => {
      this.etat = await invoke<Snapshot>('appuyer', { touche });
    });
    this.#file = suite.catch((e) => console.error('Calculatrice :', e));
    return suite;
  }

  async charger(): Promise<void> {
    this.etat = await invoke<Snapshot>('etat');
  }
}

export const calc = new Calculatrice();
