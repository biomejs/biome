{#each categories as category}
  <div class="category">
    <h2>{category.name}</h2>
    {#each category.items as item, i}
      <div class="item">{i}: {item.title}</div>
    {/each}
  </div>
{/each}

{#each matrix as row}
  {#each row as cell}
    <span>{cell}</span>
  {/each}
{/each}
