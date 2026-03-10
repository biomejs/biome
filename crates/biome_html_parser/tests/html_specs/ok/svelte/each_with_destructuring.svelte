{#each items as { id, name }}
  <div>{id}: {name}</div>
{/each}

{#each users as { name, email }, i}
  <div>{i}: {name} ({email})</div>
{/each}

{#each products as [id, title]}
  <div>{id}: {title}</div>
{/each}

{#each items as { id, ...rest }}
  <div>{id}</div>
{/each}
