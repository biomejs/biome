<!-- should not generate diagnostics -->

{const area = $derived(box.width * box.height)}

{#each boxes as box}
  {const volume = $derived(box.width * box.height * box.depth)}
  <p>{volume}</p>
{/each}
