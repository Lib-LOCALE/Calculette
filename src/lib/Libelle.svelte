<script lang="ts">
  // Affiche un libellé de touche ; `{…}` devient un exposant : `x{2}` → x².
  let { texte }: { texte: string } = $props();

  const morceaux = $derived(
    texte.split(/(\{[^}]*\})/).filter(Boolean).map((m) =>
      m.startsWith('{') ? { exposant: true, t: m.slice(1, -1) } : { exposant: false, t: m },
    ),
  );
</script>

{#each morceaux as m, i (i)}{#if m.exposant}<sup>{m.t}</sup>{:else}{m.t}{/if}{/each}

<style>
  sup {
    font-size: 0.68em;
    line-height: 0;
    vertical-align: 0.55em;
    margin-inline: 0.04em;
  }
</style>
