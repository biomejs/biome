<!-- should not generate diagnostics -->

{#each items as item (item.id)}
  <div>{item}</div>
{/each}

{#each items as item, i (item.id)}
  <div>{i}: {item}</div>
{/each}

{#each users as { id, name } (id)}
  <div>{id}: {name}</div>
{/each}

{#each items, i}
  <div>{i}</div>
{/each}
