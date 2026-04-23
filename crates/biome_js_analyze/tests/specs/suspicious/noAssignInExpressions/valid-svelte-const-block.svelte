<!-- should not generate diagnostics -->
<!-- https://github.com/biomejs/biome/issues/10082 -->
<script>
  const foo = [1, 2, 3, 4];
</script>

{#each foo as f}
  {@const bar = f ** 2}
  <div>{bar}</div>
{/each}
