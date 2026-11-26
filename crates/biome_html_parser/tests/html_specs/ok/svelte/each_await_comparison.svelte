{#each count < limit ? items : [] as item}
  <p>{item}</p>
{/each}
{#await count < limit then value}
  <p>{value}</p>
{/await}
{#each items as constant}
  <p>{constant}</p>
{/each}
