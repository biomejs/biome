{#each obj.items as item}
  <div>{item}</div>
{/each}

{#each getItems() as item}
  <div>{item}</div>
{/each}

{#each items.filter(x => x.active) as item}
  <div>{item}</div>
{/each}
