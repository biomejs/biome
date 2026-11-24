<!-- should generate diagnostics -->
<script>
  const maybeRead = 1;
  const shadowed = 2;
  function run() { const definitelyUnused = 2; }
  run();
</script>
{#each [1] as shadowed}{broken + }{/each}
{#if false}{@const sibling = 3}{/if}
