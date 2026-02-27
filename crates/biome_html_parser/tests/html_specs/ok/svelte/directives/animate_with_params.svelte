{#each list as item (item)}
  <li animate:flip={{ delay: 500 }}>{item}</li>
{/each}
{#each items as item (item.id)}
  <div animate:flip={{ duration: 300, easing: cubicOut }}>{item.name}</div>
{/each}
