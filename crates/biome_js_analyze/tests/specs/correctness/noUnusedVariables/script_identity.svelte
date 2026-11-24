<!-- should generate diagnostics -->
<script module>
  const shared = 1;
  const shadowed = 0;
</script>
<script>
  import { readable } from 'svelte/store';
  const shadowed = shared;
  const store = readable(0);
  function run() {
    const shadowed = 2;
  }
  run();
</script>
<button onclick={() => { const shadowed = 3; }}>{shadowed}</button>
<button onclick={() => { const shadowed = 4; console.log(shadowed); }}>Log</button>
{#snippet display(shadowed)}<p>{shadowed}</p>{/snippet}
{@render display(shadowed)}
<button onclick={() => { const store = 0; }}>{$store}</button>
