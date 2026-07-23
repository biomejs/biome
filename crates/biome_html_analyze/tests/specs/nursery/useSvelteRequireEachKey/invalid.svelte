<!-- should generate diagnostics -->

{#each items as item}
  <div>{item}</div>
{/each}

{#each items as item, i}
  <div>{i}: {item}</div>
{/each}

{#each users as { id, name }}
  <div>{id}: {name}</div>
{/each}
