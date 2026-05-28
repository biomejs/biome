{#each ['a', 'b'] as const as item}
  <p>{item}</p>
{/each}

{#each items as const as { id, name }}
  <p>{id}: {name}</p>
{/each}

{#each getItems() as const as item, i}
  <p>{i}: {item}</p>
{/each}
