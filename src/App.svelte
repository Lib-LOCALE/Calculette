<script lang="ts">
  import { onMount } from 'svelte';
  import { Calculator, Clock, Delete, FlaskConical, Menu, Trash2, X } from '@lucide/svelte';
  import { calc } from './lib/calc.svelte';
  import {
    DICTIONNAIRE,
    LANGUES,
    detecterLangueInitiale,
    formaterNombreLocalise,
    type Langue,
  } from './lib/i18n';
  import { SCIENTIFIQUE, STANDARD, resoudre, resoudreRaccourciClavier, type Touche } from './lib/keys';
  import Libelle from './lib/Libelle.svelte';
  import LogoAnime from './lib/LogoAnime.svelte';
  import type { Angle, Key, Mode } from './lib/types';

  type SaveurCtp = 'latte' | 'frappe' | 'macchiato' | 'mocha';
  type AccentCtp = 'mauve' | 'blue' | 'sapphire' | 'lavender' | 'peach' | 'green' | 'pink';

  const SAVEURS: { id: SaveurCtp; nom: string; apercuBg: string; apercuTexte: string }[] = [
    { id: 'latte', nom: 'Latte', apercuBg: '#eff1f5', apercuTexte: '#4c4f69' },
    { id: 'frappe', nom: 'Frappé', apercuBg: '#303446', apercuTexte: '#c6d0f5' },
    { id: 'macchiato', nom: 'Macchiato', apercuBg: '#24273a', apercuTexte: '#cad3f5' },
    { id: 'mocha', nom: 'Mocha', apercuBg: '#1e1e2e', apercuTexte: '#cdd6f4' },
  ];

  const ACCENTS: { id: AccentCtp; hex: string }[] = [
    { id: 'mauve', hex: '#cba6f7' },
    { id: 'blue', hex: '#89b4fa' },
    { id: 'sapphire', hex: '#74c7ec' },
    { id: 'lavender', hex: '#b4befe' },
    { id: 'peach', hex: '#fab387' },
    { id: 'green', hex: '#a6e3a1' },
    { id: 'pink', hex: '#f5c2e7' },
  ];

  let menuOuvert = $state(false);
  let tiroir = $state<'historique' | 'memoire' | null>(null);
  let ongletPanneau = $state<'historique' | 'memoire'>('historique');
  let seconde = $state(false);
  let hyp = $state(false);
  let toucheActive = $state<string | null>(null);
  let copieOk = $state(false);
  let saveur = $state<SaveurCtp>('mocha');
  let accent = $state<AccentCtp>('mauve');
  let langue = $state<Langue>('fr');
  let timerCopie: ReturnType<typeof setTimeout> | null = null;
  let timerFlash: ReturnType<typeof setTimeout> | null = null;

  const tr = $derived(DICTIONNAIRE[langue]);
  const etat = $derived(calc.etat);
  const estScientifique = $derived(etat.mode === 'scientifique');
  const aMemoire = $derived(etat.memory.length > 0);
  const grille = $derived(estScientifique ? SCIENTIFIQUE : STANDARD);

  const texteAffichage = $derived.by(() => {
    if (etat.error && etat.errorKind) {
      return tr.erreurs[etat.errorKind];
    }
    return formaterNombreLocalise(etat.display, tr);
  });

  const texteExpression = $derived(formaterNombreLocalise(etat.expression, tr));

  const tailleAffichage = $derived.by(() => {
    const n = texteAffichage.length;
    if (etat.error) return 'clamp(1.2rem, 4.6vw, 1.55rem)';
    if (n <= 10) return 'clamp(2.35rem, 8.5vw, 3.15rem)';
    if (n <= 14) return 'clamp(1.9rem, 6.8vw, 2.45rem)';
    if (n <= 18) return 'clamp(1.55rem, 5.4vw, 2rem)';
    return 'clamp(1.25rem, 4.4vw, 1.65rem)';
  });

  const ANGLES: { id: Angle; court: string }[] = [
    { id: 'deg', court: 'DEG' },
    { id: 'rad', court: 'RAD' },
    { id: 'grad', court: 'GRAD' },
  ];

  const angleCourant = $derived(ANGLES.find((a) => a.id === etat.angle) ?? ANGLES[0]);
  const ariaAngleCourant = $derived(
    etat.angle === 'deg'
      ? tr.ui.degVersRad
      : etat.angle === 'rad'
        ? tr.ui.radVersGrad
        : tr.ui.gradVersDeg,
  );

  function appliquerTheme(s: SaveurCtp, a: AccentCtp) {
    saveur = s;
    accent = a;
    document.documentElement.dataset.theme = s;
    document.documentElement.dataset.accent = a;
    try {
      localStorage.setItem('calculatrice.catppuccin.saveur', s);
      localStorage.setItem('calculatrice.catppuccin.accent', a);
    } catch {
      // Stockage non disponible
    }
  }

  function appliquerLangue(l: Langue) {
    langue = l;
    document.documentElement.lang = l;
    document.title = DICTIONNAIRE[l].ui.titreApp;
    try {
      localStorage.setItem('calculatrice.langue', l);
    } catch {
      // Stockage non disponible
    }
  }

  onMount(() => {
    try {
      const s = localStorage.getItem('calculatrice.catppuccin.saveur') as SaveurCtp | null;
      const a = localStorage.getItem('calculatrice.catppuccin.accent') as AccentCtp | null;
      const saveurInit = SAVEURS.some((x) => x.id === s) ? s! : 'mocha';
      const accentInit = ACCENTS.some((x) => x.id === a) ? a! : 'mauve';
      appliquerTheme(saveurInit, accentInit);
    } catch {
      appliquerTheme('mocha', 'mauve');
    }
    appliquerLangue(detecterLangueInitiale());
    calc.charger();
  });

  function libelleTouche(t: Touche): string {
    if (t.id === 'dec') return tr.separateurDecimal;
    if (t.id === 'seconde') return tr.toucheSeconde;
    return t.label;
  }

  function ariaTouche(t: Touche): string {
    return tr.touches[t.id] ?? t.aria;
  }

  function flasher(id: string) {
    toucheActive = id;
    if (timerFlash) clearTimeout(timerFlash);
    timerFlash = setTimeout(() => {
      toucheActive = null;
    }, 130);
  }

  function actionnerTouche(t: Touche) {
    if (etat.error && !t.sure) return;
    if (t.bascule === 'seconde') {
      seconde = !seconde;
      return;
    }
    if (t.bascule === 'hyp') {
      hyp = !hyp;
      return;
    }
    if (t.key) {
      calc.appuyer(t.key);
    }
  }

  function choisirMode(m: Mode) {
    menuOuvert = false;
    seconde = false;
    hyp = false;
    calc.appuyer({ type: 'setMode', value: m });
  }

  function cycleAngle() {
    const idx = ANGLES.findIndex((a) => a.id === etat.angle);
    const suivant = ANGLES[(idx + 1) % ANGLES.length].id;
    calc.appuyer({ type: 'setAngle', value: suivant });
  }

  function appuyerMemoire(
    type: 'memoryClear' | 'memoryRecall' | 'memoryAdd' | 'memorySub',
    index: number | null = null,
  ) {
    if (etat.error) return;
    calc.appuyer({ type, value: index });
  }

  async function copierAffichage() {
    if (etat.error) return;
    try {
      await navigator.clipboard.writeText(texteAffichage.replace(/\u202F/g, ''));
      copieOk = true;
      if (timerCopie) clearTimeout(timerCopie);
      timerCopie = setTimeout(() => {
        copieOk = false;
      }, 1200);
    } catch {
      // Presse-papiers indisponible
    }
  }

  function trouverToucheParId(id: string): Touche | null {
    for (const ligne of grille) {
      for (const c of ligne) {
        const t = resoudre(c, seconde, hyp);
        if (t.id === id) return t;
        if (c.base.id === id) return c.base;
      }
    }
    return null;
  }

  function surClavier(e: KeyboardEvent) {
    if (e.key === 'Escape' && (menuOuvert || tiroir !== null)) {
      e.preventDefault();
      menuOuvert = false;
      tiroir = null;
      return;
    }

    if ((e.ctrlKey || e.metaKey) && !e.altKey) {
      const k = e.key.toLowerCase();
      if (k === 'c') {
        e.preventDefault();
        copierAffichage();
        return;
      }
      if (k === 'v') {
        e.preventDefault();
        navigator.clipboard
          ?.readText()
          .then((texte) => {
            if (texte) calc.appuyer({ type: 'paste', value: texte });
          })
          .catch(() => {});
        return;
      }
      if (k === 'l') {
        e.preventDefault();
        if (aMemoire) appuyerMemoire('memoryClear');
        return;
      }
      if (k === 'r') {
        e.preventDefault();
        if (aMemoire) appuyerMemoire('memoryRecall');
        return;
      }
      if (k === 'm') {
        e.preventDefault();
        if (!etat.error) calc.appuyer({ type: 'memoryStore' });
        return;
      }
      if (k === 'p') {
        e.preventDefault();
        appuyerMemoire('memoryAdd');
        return;
      }
      if (k === 'q') {
        e.preventDefault();
        appuyerMemoire('memorySub');
        return;
      }
      return;
    }

    if (e.altKey && (e.key === '1' || e.key === '2' || e.code === 'Digit1' || e.code === 'Digit2')) {
      e.preventDefault();
      choisirMode(e.key === '1' || e.code === 'Digit1' ? 'standard' : 'scientifique');
      return;
    }

    if (e.ctrlKey || e.metaKey || e.altKey) return;

    if (e.key === 'h' || e.key === 'H') {
      e.preventDefault();
      tiroir = tiroir === 'historique' ? null : 'historique';
      ongletPanneau = 'historique';
      return;
    }

    const id = resoudreRaccourciClavier(e);
    if (!id) return;

    e.preventDefault();
    const t = trouverToucheParId(id);
    if (t) {
      flasher(t.id);
      actionnerTouche(t);
    } else {
      const secours: Record<string, Key> = {
        clear: { type: 'clear' },
        ce: { type: 'clearEntry' },
        back: { type: 'backspace' },
        eq: { type: 'equals' },
        neg: { type: 'unary', value: 'negate' },
        inv: { type: 'unary', value: 'inverse' },
        sqr: { type: 'unary', value: 'square' },
        sqrt: { type: 'unary', value: 'sqrt' },
      };
      if (secours[id]) calc.appuyer(secours[id]);
    }
  }

  function surColler(e: ClipboardEvent) {
    const texte = e.clipboardData?.getData('text');
    if (texte) {
      e.preventDefault();
      calc.appuyer({ type: 'paste', value: texte });
    }
  }
</script>

<svelte:window onkeydown={surClavier} onpaste={surColler} />

<div class="layout">
  <main class="calc">
    <!-- Barre supérieure -->
    <header class="topbar">
      <div class="topbar-gauche">
        <button
          type="button"
          class="icon-btn"
          aria-label={tr.ui.menuEtParametres}
          aria-expanded={menuOuvert}
          onclick={() => (menuOuvert = !menuOuvert)}
        >
          <Menu size={18} strokeWidth={1.9} />
        </button>
        <LogoAnime taille={21} />
        <h1 class="mode-titre">{tr.modes[etat.mode]}</h1>
      </div>

      <button
        type="button"
        class="icon-btn btn-historique-mobile"
        aria-label={tr.ui.afficherHistorique}
        aria-pressed={tiroir === 'historique'}
        onclick={() => (tiroir = tiroir === 'historique' ? null : 'historique')}
      >
        <Clock size={18} strokeWidth={1.9} />
      </button>
    </header>

    <!-- Zone d'affichage -->
    <section class="ecran" aria-label={tr.ui.affichage}>
      <div class="expression" title={texteExpression}>
        {texteExpression || '\u00A0'}
      </div>
      <div class="resultat-ligne">
        <output
          class="resultat"
          class:erreur={etat.error}
          style:font-size={tailleAffichage}
          aria-live="polite"
          ondblclick={copierAffichage}
          title={tr.ui.copierAstuce}
        >
          {texteAffichage}
        </output>
        {#if copieOk}
          <span class="badge-copie" role="status">{tr.ui.copie}</span>
        {/if}
      </div>
    </section>

    <!-- Barre d'angle (mode Scientifique uniquement) -->
    {#if estScientifique}
      <div class="barre-sci">
        <button
          type="button"
          class="chip-sci"
          aria-label={ariaAngleCourant}
          onclick={cycleAngle}
        >
          {angleCourant.court}
        </button>
      </div>
    {/if}

    <!-- Barre de mémoire -->
    <div class="barre-memoire" role="toolbar" aria-label={tr.ui.memoire}>
      <button
        type="button"
        class="mem-btn"
        disabled={!aMemoire || etat.error}
        title={tr.ui.mcTitre}
        aria-label={tr.ui.mcTitre}
        onclick={() => appuyerMemoire('memoryClear')}
      >
        MC
      </button>
      <button
        type="button"
        class="mem-btn"
        disabled={!aMemoire || etat.error}
        title={tr.ui.mrTitre}
        aria-label={tr.ui.mrTitre}
        onclick={() => appuyerMemoire('memoryRecall')}
      >
        MR
      </button>
      <button
        type="button"
        class="mem-btn"
        disabled={etat.error}
        title={tr.ui.mPlusTitre}
        aria-label={tr.ui.mPlusTitre}
        onclick={() => appuyerMemoire('memoryAdd')}
      >
        M+
      </button>
      <button
        type="button"
        class="mem-btn"
        disabled={etat.error}
        title={tr.ui.mMoinsTitre}
        aria-label={tr.ui.mMoinsTitre}
        onclick={() => appuyerMemoire('memorySub')}
      >
        M−
      </button>
      <button
        type="button"
        class="mem-btn"
        disabled={etat.error}
        title={tr.ui.msTitre}
        aria-label={tr.ui.msTitre}
        onclick={() => calc.appuyer({ type: 'memoryStore' })}
      >
        MS
      </button>
      <button
        type="button"
        class="mem-btn mem-btn-tiroir"
        disabled={!aMemoire}
        title={tr.ui.afficherMemoire}
        aria-label={tr.ui.afficherMemoire}
        onclick={() => (tiroir = tiroir === 'memoire' ? null : 'memoire')}
      >
        M▾
      </button>
    </div>

    <!-- Pavé de touches -->
    <div
      class="pave"
      class:pave-sci={estScientifique}
      role="group"
      aria-label={tr.ui.clavier}
    >
      {#each grille as ligne}
        {#each ligne as c}
          {@const t = resoudre(c, seconde, hyp)}
          {@const desactive = etat.error && !t.sure}
          {@const actifBascule =
            (t.bascule === 'seconde' && seconde) || (t.bascule === 'hyp' && hyp)}
          <button
            type="button"
            class="touche {t.genre}"
            class:active={toucheActive === t.id}
            class:bascule-on={actifBascule}
            disabled={desactive}
            aria-label={ariaTouche(t)}
            aria-pressed={t.bascule ? actifBascule : undefined}
            onclick={() => actionnerTouche(t)}
          >
            {#if t.id === 'back'}
              <Delete size={19} strokeWidth={1.8} />
            {:else if t.id === 'open' && etat.openParens > 0}
              <span>(<sub class="indice-paren">{etat.openParens}</sub></span>
            {:else}
              <Libelle texte={libelleTouche(t)} />
            {/if}
          </button>
        {/each}
      {/each}
    </div>

    <!-- Menu latéral (Modes + Thèmes Catppuccin + Langues) -->
    {#if menuOuvert}
      <div
        class="scrim"
        role="presentation"
        onclick={() => (menuOuvert = false)}
      ></div>
      <nav class="menu-nav" aria-label={tr.ui.menuEtParametres}>
        <div class="menu-entete">
          <button
            type="button"
            class="icon-btn"
            aria-label={tr.ui.fermerMenu}
            onclick={() => (menuOuvert = false)}
          >
            <Menu size={18} strokeWidth={1.9} />
          </button>
          <span class="menu-categorie">{tr.ui.categorieCalculateur}</span>
        </div>

        <button
          type="button"
          class="menu-item"
          class:selectionne={etat.mode === 'standard'}
          onclick={() => choisirMode('standard')}
        >
          <Calculator size={18} strokeWidth={1.8} />
          <span>{tr.modes.standard}</span>
          <kbd>Alt+1</kbd>
        </button>

        <button
          type="button"
          class="menu-item"
          class:selectionne={etat.mode === 'scientifique'}
          onclick={() => choisirMode('scientifique')}
        >
          <FlaskConical size={18} strokeWidth={1.8} />
          <span>{tr.modes.scientifique}</span>
          <kbd>Alt+2</kbd>
        </button>

        <div class="menu-sep"></div>
        <span class="menu-sous-titre">{tr.ui.themeCatppuccin}</span>

        <div class="grille-saveurs">
          {#each SAVEURS as s (s.id)}
            <button
              type="button"
              class="btn-saveur"
              class:selectionne={saveur === s.id}
              onclick={() => appliquerTheme(s.id, accent)}
            >
              <span
                class="pastille-saveur"
                style:background={s.apercuBg}
                style:border-color={s.apercuTexte}
              ></span>
              <span>{s.nom}</span>
            </button>
          {/each}
        </div>

        <span class="menu-sous-titre">{tr.ui.couleurAccent}</span>
        <div class="barre-accents" role="group" aria-label={tr.ui.couleurAccent}>
          {#each ACCENTS as a (a.id)}
            <button
              type="button"
              class="pastille-accent"
              class:actif={accent === a.id}
              style:background={a.hex}
              title={tr.accents[a.id]}
              aria-label={tr.accents[a.id]}
              aria-pressed={accent === a.id}
              onclick={() => appliquerTheme(saveur, a.id)}
            ></button>
          {/each}
        </div>

        <div class="menu-sep"></div>
        <span class="menu-sous-titre">{tr.ui.langue}</span>
        <div class="grille-langues" role="group" aria-label={tr.ui.langue}>
          {#each LANGUES as l (l.code)}
            <button
              type="button"
              class="btn-langue"
              class:selectionne={langue === l.code}
              onclick={() => appliquerLangue(l.code)}
            >
              <span class="drapeau" aria-hidden="true">{l.drapeau}</span>
              <span>{l.nom}</span>
            </button>
          {/each}
        </div>
      </nav>
    {/if}

    <!-- Tiroir bas (Historique / Mémoire sur fenêtre compacte) -->
    {#if tiroir !== null}
      <div
        class="scrim scrim-tiroir"
        role="presentation"
        onclick={() => (tiroir = null)}
      ></div>
      <section
        class="tiroir"
        aria-label={tiroir === 'historique' ? tr.ui.historique : tr.ui.memoire}
      >
        <header class="tiroir-entete">
          <div class="onglets">
            <button
              type="button"
              class="onglet"
              class:actif={tiroir === 'historique'}
              onclick={() => (tiroir = 'historique')}
            >
              {tr.ui.historique}
            </button>
            <button
              type="button"
              class="onglet"
              class:actif={tiroir === 'memoire'}
              onclick={() => (tiroir = 'memoire')}
            >
              {tr.ui.memoire}
            </button>
          </div>
          <button
            type="button"
            class="icon-btn"
            aria-label={tr.ui.fermerPanneau}
            onclick={() => (tiroir = null)}
          >
            <X size={18} strokeWidth={1.9} />
          </button>
        </header>

        <div class="tiroir-corps">
          {#if tiroir === 'historique'}
            {#if etat.history.length === 0}
              <p class="vide">{tr.ui.aucunHistorique}</p>
            {:else}
              <ul class="liste-hist">
                {#each etat.history as item, i (i)}
                  <li>
                    <button
                      type="button"
                      class="hist-item"
                      onclick={() => {
                        calc.appuyer({ type: 'historyRecall', value: i });
                        tiroir = null;
                      }}
                    >
                      <span class="hist-expr">{formaterNombreLocalise(item.expression, tr)}</span>
                      <span class="hist-res">{formaterNombreLocalise(item.result, tr)}</span>
                    </button>
                  </li>
                {/each}
              </ul>
            {/if}
          {:else if etat.memory.length === 0}
            <p class="vide">{tr.ui.aucuneMemoire}</p>
          {:else}
            <ul class="liste-mem">
              {#each etat.memory as val, i (i)}
                <li class="mem-item">
                  <button
                    type="button"
                    class="mem-val"
                    onclick={() => {
                      appuyerMemoire('memoryRecall', i);
                      tiroir = null;
                    }}
                  >
                    {formaterNombreLocalise(val, tr)}
                  </button>
                  <div class="mem-actions">
                    <button
                      type="button"
                      class="mini-btn"
                      aria-label={tr.ui.mcEntree}
                      onclick={() => appuyerMemoire('memoryClear', i)}
                    >
                      MC
                    </button>
                    <button
                      type="button"
                      class="mini-btn"
                      aria-label={tr.ui.mPlusEntree}
                      onclick={() => appuyerMemoire('memoryAdd', i)}
                    >
                      M+
                    </button>
                    <button
                      type="button"
                      class="mini-btn"
                      aria-label={tr.ui.mMoinsEntree}
                      onclick={() => appuyerMemoire('memorySub', i)}
                    >
                      M−
                    </button>
                  </div>
                </li>
              {/each}
            </ul>
          {/if}
        </div>

        {#if (tiroir === 'historique' && etat.history.length > 0) || (tiroir === 'memoire' && etat.memory.length > 0)}
          <footer class="tiroir-pied">
            <button
              type="button"
              class="icon-btn"
              aria-label={tiroir === 'historique' ? tr.ui.effacerHistorique : tr.ui.effacerMemoire}
              title={tiroir === 'historique' ? tr.ui.effacerHistorique : tr.ui.effacerMemoire}
              onclick={() =>
                tiroir === 'historique'
                  ? calc.appuyer({ type: 'historyClear' })
                  : appuyerMemoire('memoryClear')}
            >
              <Trash2 size={17} strokeWidth={1.8} />
            </button>
          </footer>
        {/if}
      </section>
    {/if}
  </main>

  <!-- Panneau latéral permanent sur fenêtre large (≥ 640px) -->
  <aside class="panneau-lateral" aria-label="{tr.ui.historique} / {tr.ui.memoire}">
    <header class="panneau-entete">
      <div class="onglets">
        <button
          type="button"
          class="onglet"
          class:actif={ongletPanneau === 'historique'}
          onclick={() => (ongletPanneau = 'historique')}
        >
          {tr.ui.historique}
        </button>
        <button
          type="button"
          class="onglet"
          class:actif={ongletPanneau === 'memoire'}
          onclick={() => (ongletPanneau = 'memoire')}
        >
          {tr.ui.memoire}
        </button>
      </div>
    </header>

    <div class="panneau-corps">
      {#if ongletPanneau === 'historique'}
        {#if etat.history.length === 0}
          <p class="vide">{tr.ui.aucunHistorique}</p>
        {:else}
          <ul class="liste-hist">
            {#each etat.history as item, i (i)}
              <li>
                <button
                  type="button"
                  class="hist-item"
                  onclick={() => calc.appuyer({ type: 'historyRecall', value: i })}
                >
                  <span class="hist-expr">{formaterNombreLocalise(item.expression, tr)}</span>
                  <span class="hist-res">{formaterNombreLocalise(item.result, tr)}</span>
                </button>
              </li>
            {/each}
          </ul>
        {/if}
      {:else if etat.memory.length === 0}
        <p class="vide">{tr.ui.aucuneMemoire}</p>
      {:else}
        <ul class="liste-mem">
          {#each etat.memory as val, i (i)}
            <li class="mem-item">
              <button
                type="button"
                class="mem-val"
                onclick={() => appuyerMemoire('memoryRecall', i)}
              >
                {formaterNombreLocalise(val, tr)}
              </button>
              <div class="mem-actions">
                <button
                  type="button"
                  class="mini-btn"
                  aria-label={tr.ui.mcEntree}
                  onclick={() => appuyerMemoire('memoryClear', i)}
                >
                  MC
                </button>
                <button
                  type="button"
                  class="mini-btn"
                  aria-label={tr.ui.mPlusEntree}
                  onclick={() => appuyerMemoire('memoryAdd', i)}
                >
                  M+
                </button>
                <button
                  type="button"
                  class="mini-btn"
                  aria-label={tr.ui.mMoinsEntree}
                  onclick={() => appuyerMemoire('memorySub', i)}
                >
                  M−
                </button>
              </div>
            </li>
          {/each}
        </ul>
      {/if}
    </div>

    {#if (ongletPanneau === 'historique' && etat.history.length > 0) || (ongletPanneau === 'memoire' && etat.memory.length > 0)}
      <footer class="panneau-pied">
        <button
          type="button"
          class="icon-btn"
          aria-label={ongletPanneau === 'historique' ? tr.ui.effacerHistorique : tr.ui.effacerMemoire}
          title={ongletPanneau === 'historique' ? tr.ui.effacerHistorique : tr.ui.effacerMemoire}
          onclick={() =>
            ongletPanneau === 'historique'
              ? calc.appuyer({ type: 'historyClear' })
              : appuyerMemoire('memoryClear')}
        >
          <Trash2 size={17} strokeWidth={1.8} />
        </button>
      </footer>
    {/if}
  </aside>
</div>

<style>
  .layout {
    display: grid;
    grid-template-columns: 1fr;
    height: 100%;
    width: 100%;
  }

  .calc {
    position: relative;
    display: flex;
    flex-direction: column;
    height: 100%;
    min-width: 0;
    padding: 6px 4px 4px;
  }

  /* Barre supérieure */
  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 40px;
    padding: 0 4px;
    flex-shrink: 0;
  }

  .topbar-gauche {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .mode-titre {
    margin: 0;
    font-size: 1.15rem;
    font-weight: 600;
    letter-spacing: -0.01em;
  }

  .icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: 6px;
    color: var(--text);
    transition: background-color 100ms ease;
  }

  .icon-btn:hover {
    background: var(--hover-subtle);
  }

  .icon-btn:active {
    background: var(--active-subtle);
  }

  /* Écran */
  .ecran {
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    min-height: 98px;
    padding: 6px 12px 8px;
    text-align: right;
    flex-shrink: 0;
  }

  .expression {
    min-height: 1.35rem;
    font-size: 0.9rem;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .resultat-ligne {
    position: relative;
    display: flex;
    align-items: baseline;
    justify-content: flex-end;
    min-height: 3.4rem;
  }

  .resultat {
    display: block;
    width: 100%;
    font-weight: 600;
    line-height: 1.12;
    letter-spacing: -0.02em;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    user-select: text;
    cursor: copy;
  }

  .resultat.erreur {
    font-weight: 600;
    letter-spacing: -0.01em;
    white-space: normal;
    color: var(--ctp-red);
  }

  .badge-copie {
    position: absolute;
    top: -18px;
    right: 0;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 0.72rem;
    font-weight: 600;
    background: var(--accent);
    color: var(--on-accent);
  }

  /* Barre scientifique (DEG / RAD / GRAD) */
  .barre-sci {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 0 6px 2px;
    flex-shrink: 0;
  }

  .chip-sci {
    height: 28px;
    padding: 0 10px;
    border-radius: 5px;
    font-size: 0.76rem;
    font-weight: 600;
    letter-spacing: 0.03em;
  }

  .chip-sci:hover {
    background: var(--hover-subtle);
  }

  .chip-sci:active {
    background: var(--active-subtle);
  }

  /* Barre mémoire */
  .barre-memoire {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 2px;
    padding: 2px 4px 4px;
    flex-shrink: 0;
  }

  .mem-btn {
    height: 30px;
    border-radius: 5px;
    font-size: 0.76rem;
    font-weight: 600;
    color: var(--text);
    transition: background-color 90ms ease;
  }

  .mem-btn:hover:not(:disabled) {
    background: var(--hover-subtle);
  }

  .mem-btn:active:not(:disabled) {
    background: var(--active-subtle);
  }

  .mem-btn:disabled {
    color: var(--text-disabled);
  }

  /* Pavé de touches */
  .pave {
    flex: 1;
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    grid-template-rows: repeat(6, 1fr);
    gap: 3px;
    min-height: 0;
  }

  .pave.pave-sci {
    grid-template-columns: repeat(5, 1fr);
    grid-template-rows: repeat(8, 1fr);
  }

  .touche {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 40px;
    border-radius: 6px;
    border: 1px solid var(--border-key);
    border-bottom-color: var(--border-bottom-key);
    box-shadow: var(--shadow-key);
    font-size: 1.05rem;
    transition: background-color 80ms ease, transform 60ms ease;
  }

  .pave-sci .touche {
    font-size: 0.94rem;
    min-height: 34px;
  }

  .touche.chiffre {
    background: var(--surface-num);
    font-size: 1.22rem;
    font-weight: 600;
  }

  .pave-sci .touche.chiffre {
    font-size: 1.12rem;
  }

  .touche.chiffre:hover:not(:disabled) {
    background: var(--surface-num-hover);
  }

  .touche.chiffre:active:not(:disabled),
  .touche.chiffre.active:not(:disabled) {
    background: var(--surface-num-active);
  }

  .touche.fonction {
    background: var(--surface-fn);
    font-weight: 450;
  }

  .touche.fonction:hover:not(:disabled) {
    background: var(--surface-fn-hover);
  }

  .touche.fonction:active:not(:disabled),
  .touche.fonction.active:not(:disabled) {
    background: var(--surface-fn-active);
  }

  .touche.bascule-on {
    background: var(--accent);
    color: var(--on-accent);
    border-color: transparent;
    font-weight: 600;
  }

  .touche.bascule-on:hover:not(:disabled) {
    background: var(--accent-hover);
  }

  .touche.egal {
    background: var(--accent);
    color: var(--on-accent);
    border-color: transparent;
    font-size: 1.3rem;
    font-weight: 600;
  }

  .touche.egal:hover:not(:disabled) {
    background: var(--accent-hover);
  }

  .touche.egal:active:not(:disabled),
  .touche.egal.active:not(:disabled) {
    background: var(--accent-active);
  }

  .touche:disabled {
    opacity: 0.38;
  }

  .indice-paren {
    font-size: 0.68em;
    margin-left: 1px;
  }

  /* Menu de navigation et tiroir */
  .scrim {
    position: absolute;
    inset: 0;
    background: var(--scrim);
    z-index: 20;
  }

  .menu-nav {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    width: 268px;
    padding: 6px 8px 12px;
    background: var(--bg-drawer);
    box-shadow: var(--shadow-elevated);
    border-right: 1px solid var(--divider);
    z-index: 30;
    display: flex;
    flex-direction: column;
    gap: 2px;
    overflow-y: auto;
  }

  .menu-entete {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 40px;
    padding: 0 2px;
    flex-shrink: 0;
  }

  .menu-categorie {
    font-size: 0.78rem;
    font-weight: 600;
    color: var(--text-muted);
  }

  .menu-sep {
    height: 1px;
    margin: 6px 4px;
    background: var(--divider);
    flex-shrink: 0;
  }

  .menu-sous-titre {
    padding: 4px 10px 4px;
    font-size: 0.74rem;
    font-weight: 600;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .grille-saveurs,
  .grille-langues {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 5px;
    padding: 0 6px 6px;
    flex-shrink: 0;
  }

  .btn-saveur,
  .btn-langue {
    display: flex;
    align-items: center;
    gap: 7px;
    height: 34px;
    padding: 0 9px;
    border-radius: 6px;
    font-size: 0.81rem;
    font-weight: 500;
    background: var(--surface-fn);
    border: 1px solid var(--border-key);
    text-align: left;
  }

  .btn-saveur:hover,
  .btn-langue:hover {
    background: var(--surface-fn-hover);
  }

  .btn-saveur.selectionne,
  .btn-langue.selectionne {
    border-color: var(--accent);
    background: var(--accent-subtle);
    font-weight: 600;
  }

  .pastille-saveur {
    width: 13px;
    height: 13px;
    border-radius: 50%;
    border: 2px solid;
    flex-shrink: 0;
  }

  .drapeau {
    font-size: 0.95rem;
    line-height: 1;
    flex-shrink: 0;
  }

  .barre-accents {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 2px 8px 6px;
    flex-shrink: 0;
  }

  .pastille-accent {
    width: 23px;
    height: 23px;
    border-radius: 50%;
    border: 2px solid transparent;
    transition: transform 90ms ease;
  }

  .pastille-accent:hover {
    transform: scale(1.12);
  }

  .pastille-accent.actif {
    border-color: var(--text);
    transform: scale(1.14);
  }

  .menu-item {
    position: relative;
    display: flex;
    align-items: center;
    gap: 12px;
    height: 38px;
    padding: 0 12px;
    border-radius: 6px;
    font-size: 0.9rem;
    text-align: left;
    flex-shrink: 0;
  }

  .menu-item:hover {
    background: var(--hover-subtle);
  }

  .menu-item.selectionne {
    background: var(--accent-subtle);
    font-weight: 600;
  }

  .menu-item.selectionne::before {
    content: '';
    position: absolute;
    left: 3px;
    top: 9px;
    bottom: 9px;
    width: 3px;
    border-radius: 2px;
    background: var(--accent);
  }

  .menu-item kbd {
    margin-left: auto;
    font-family: inherit;
    font-size: 0.74rem;
    color: var(--text-muted);
  }

  /* Tiroir bas (fenêtre étroite) */
  .tiroir {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    height: 68%;
    background: var(--bg-drawer);
    border-top: 1px solid var(--divider);
    border-radius: 10px 10px 0 0;
    box-shadow: var(--shadow-elevated);
    z-index: 25;
    display: flex;
    flex-direction: column;
  }

  .tiroir-entete,
  .panneau-entete {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px 4px;
    flex-shrink: 0;
  }

  .onglets {
    display: flex;
    gap: 16px;
  }

  .onglet {
    position: relative;
    padding: 6px 2px 8px;
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--text-muted);
  }

  .onglet.actif {
    color: var(--text);
  }

  .onglet.actif::after {
    content: '';
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    height: 3px;
    border-radius: 2px;
    background: var(--accent);
  }

  .tiroir-corps,
  .panneau-corps {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
  }

  .tiroir-pied,
  .panneau-pied {
    display: flex;
    justify-content: flex-end;
    padding: 4px 8px 6px;
    border-top: 1px solid var(--divider);
  }

  .vide {
    margin: 16px 8px;
    font-size: 0.88rem;
    color: var(--text-muted);
  }

  .liste-hist,
  .liste-mem {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .hist-item {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 2px;
    padding: 8px 12px;
    border-radius: 6px;
    text-align: right;
  }

  .hist-item:hover {
    background: var(--hover-subtle);
  }

  .hist-expr {
    font-size: 0.8rem;
    color: var(--text-muted);
    word-break: break-all;
  }

  .hist-res {
    font-size: 1.2rem;
    font-weight: 600;
  }

  .mem-item {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    padding: 8px 12px;
    border-radius: 6px;
  }

  .mem-item:hover {
    background: var(--hover-subtle);
  }

  .mem-val {
    width: 100%;
    text-align: right;
    font-size: 1.2rem;
    font-weight: 600;
    padding: 2px 0;
  }

  .mem-actions {
    display: flex;
    gap: 4px;
    margin-top: 4px;
  }

  .mini-btn {
    padding: 3px 8px;
    border-radius: 4px;
    font-size: 0.72rem;
    font-weight: 600;
    background: var(--surface-num);
    border: 1px solid var(--border-key);
  }

  .mini-btn:hover {
    background: var(--surface-num-hover);
  }

  /* Panneau latéral large (≥ 640px) */
  .panneau-lateral {
    display: none;
    flex-direction: column;
    background: var(--bg-panel);
    border-left: 1px solid var(--divider);
    min-width: 0;
  }

  @media (min-width: 640px) {
    .layout {
      grid-template-columns: minmax(320px, 1fr) 260px;
    }

    .panneau-lateral {
      display: flex;
    }

    .btn-historique-mobile,
    .mem-btn-tiroir,
    .tiroir,
    .scrim-tiroir {
      display: none;
    }
  }
</style>
