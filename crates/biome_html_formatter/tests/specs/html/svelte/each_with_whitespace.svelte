{#each  items  as  item  }
  <div>{item}</div>
{/each}

{#each items as item,i}
  <div>{item}</div>
{/each}

{#each items as item ( item.id ) }
  <div>{item}</div>
{/each}
