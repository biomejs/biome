{#each items as item, i (item.id)}
  <div>{i}: {item.name}</div>
{:else}
  <div>Empty list</div>
{/each}

{#each users, index}
  <div>User #{index}</div>
{/each}

{#each products as product (product.sku)}
  <div>{product.title}</div>
{/each}
