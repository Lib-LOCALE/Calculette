<script lang="ts">
  // Icône vectorielle animée générée avec VectorCraft + EffectCraft,
  // synchronisée avec les variables CSS du thème Catppuccin actif.
  let { taille = 22 }: { taille?: number } = $props();
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  viewBox="0 0 256 256"
  width={taille}
  height={taille}
  class="logo-calc"
  aria-hidden="true"
>
  <path
    d="M66 12 L190 12 C213.196 12 232 30.804 232 54 L232 202 C232 225.196 213.196 244 190 244 L66 244 C42.804 244 24 225.196 24 202 L24 54 C24 30.804 42.804 12 66 12 Z"
    class="corps"
    stroke-width="6"
  />
  <path
    d="M62 34 L194 34 C202.837 34 210 41.163 210 50 L210 82 C210 90.837 202.837 98 194 98 L62 98 C53.163 98 46 90.837 46 82 L46 50 C46 41.163 53.163 34 62 34 Z"
    class="ecran"
  />
  <path
    d="M62 35 L136 35 L106 97 L62 97 C53.716 97 47 90.284 47 82 L47 50 C47 41.716 53.716 35 62 35 Z"
    class="reflet"
  />
  <path
    d="M112 60 L128 60 C131.314 60 134 62.686 134 66 L134 76 C134 79.314 131.314 82 128 82 L112 82 C108.686 82 106 79.314 106 76 L106 66 C106 62.686 108.686 60 112 60 Z"
    class="chiffre"
    opacity="0.88"
  />
  <path
    d="M146 60 L166 60 C169.314 60 172 62.686 172 66 L172 76 C172 79.314 169.314 82 166 82 L146 82 C142.686 82 140 79.314 140 76 L140 66 C140 62.686 142.686 60 146 60 Z"
    class="chiffre"
  />
  <path
    d="M185.5 56 C187.433 56 189 57.567 189 59.5 L189 82.5 C189 84.433 187.433 86 185.5 86 C183.567 86 182 84.433 182 82.5 L182 59.5 C182 57.567 183.567 56 185.5 56 Z"
    class="curseur"
  />
  <g class="t1">
    <path
      d="M56 114 L84 114 C89.523 114 94 118.477 94 124 L94 138 C94 143.523 89.523 148 84 148 L56 148 C50.477 148 46 143.523 46 138 L46 124 C46 118.477 50.477 114 56 114 Z"
      class="touche-op"
    />
    <path d="M70 124 L70 138 M63 131 L77 131" class="symbole" stroke-width="5" stroke-linecap="round" />
  </g>
  <g class="t2">
    <path
      d="M114 114 L142 114 C147.523 114 152 118.477 152 124 L152 138 C152 143.523 147.523 148 142 148 L114 148 C108.477 148 104 143.523 104 138 L104 124 C104 118.477 108.477 114 114 114 Z"
      class="touche-op"
    />
    <path d="M121 131 L135 131" class="symbole" stroke-width="5" stroke-linecap="round" />
  </g>
  <path
    d="M172 114 L200 114 C205.523 114 210 118.477 210 124 L210 138 C210 143.523 205.523 148 200 148 L172 148 C166.477 148 162 143.523 162 138 L162 124 C162 118.477 166.477 114 172 114 Z"
    class="touche-op"
  />
  <path d="M180 125 L192 137 M192 125 L180 137" class="symbole-accent" stroke-width="5" stroke-linecap="round" />
  <path
    d="M56 158 L84 158 C89.523 158 94 162.477 94 168 L94 180 C94 185.523 89.523 190 84 190 L56 190 C50.477 190 46 185.523 46 180 L46 168 C46 162.477 50.477 158 56 158 Z"
    class="touche-num"
  />
  <g class="t3">
    <path
      d="M114 158 L142 158 C147.523 158 152 162.477 152 168 L152 180 C152 185.523 147.523 190 142 190 L114 190 C108.477 190 104 185.523 104 180 L104 168 C104 162.477 108.477 158 114 158 Z"
      class="touche-num"
    />
  </g>
  <path
    d="M55 200 L85 200 C89.971 200 94 204.029 94 209 L94 217 C94 221.971 89.971 226 85 226 L55 226 C50.029 226 46 221.971 46 217 L46 209 C46 204.029 50.029 200 55 200 Z"
    class="touche-num"
  />
  <path
    d="M113 200 L143 200 C147.971 200 152 204.029 152 209 L152 217 C152 221.971 147.971 226 143 226 L113 226 C108.029 226 104 221.971 104 217 L104 209 C104 204.029 108.029 200 113 200 Z"
    class="touche-num"
  />
  <g class="egal-btn">
    <path
      d="M174 158 L198 158 C204.627 158 210 163.373 210 170 L210 214 C210 220.627 204.627 226 198 226 L174 226 C167.373 226 162 220.627 162 214 L162 170 C162 163.373 167.373 158 174 158 Z"
      class="egal"
    />
    <path d="M176 186 L196 186 M176 198 L196 198" class="egal-symbole" stroke-width="6" stroke-linecap="round" />
  </g>
</svg>

<style>
  .logo-calc {
    display: block;
    flex-shrink: 0;
  }
  .corps {
    fill: var(--ctp-surface0);
    stroke: var(--ctp-surface2);
  }
  .ecran {
    fill: var(--ctp-crust);
  }
  .reflet {
    fill: var(--ctp-blue);
    opacity: 0.14;
  }
  .chiffre {
    fill: var(--ctp-text);
  }
  .curseur {
    fill: var(--accent);
    animation: ctp-curseur 1.2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }
  .touche-op {
    fill: var(--ctp-surface1);
  }
  .touche-num {
    fill: var(--ctp-surface2);
  }
  .symbole {
    fill: none;
    stroke: var(--ctp-text);
  }
  .symbole-accent {
    fill: none;
    stroke: var(--accent);
  }
  .egal {
    fill: var(--accent);
  }
  .egal-symbole {
    fill: none;
    stroke: var(--on-accent);
  }
  .t1 {
    transform-box: fill-box;
    transform-origin: center;
    animation: ctp-touche 3.6s cubic-bezier(0.4, 0, 0.2, 1) infinite 0.2s;
  }
  .t2 {
    transform-box: fill-box;
    transform-origin: center;
    animation: ctp-touche 3.6s cubic-bezier(0.4, 0, 0.2, 1) infinite 1s;
  }
  .t3 {
    transform-box: fill-box;
    transform-origin: center;
    animation: ctp-touche 3.6s cubic-bezier(0.4, 0, 0.2, 1) infinite 1.8s;
  }
  .egal-btn {
    transform-box: fill-box;
    transform-origin: center;
    animation: ctp-egal 3.6s cubic-bezier(0.4, 0, 0.2, 1) infinite 2.5s;
  }
  @keyframes ctp-curseur {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.2; }
  }
  @keyframes ctp-touche {
    0%, 16%, 100% { transform: scale(1); opacity: 1; }
    8% { transform: scale(0.9); opacity: 0.72; }
  }
  @keyframes ctp-egal {
    0%, 20%, 100% { transform: scale(1); filter: brightness(1); }
    10% { transform: scale(0.93); filter: brightness(1.16); }
  }
</style>
