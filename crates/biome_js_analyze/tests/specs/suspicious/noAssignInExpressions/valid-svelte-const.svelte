<!-- should not generate diagnostics -->
<script lang="ts">
  const numbers = [1, 2, 3, 4];
</script>

{#each numbers as number, index (number)}
  {@const n = number + 1}
  <p>{n}</p>
{/each}
