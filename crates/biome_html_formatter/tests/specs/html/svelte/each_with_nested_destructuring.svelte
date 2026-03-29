{#each Object.entries(foo) as [key, { a, b }]}
  <div>{key}: {a} {b}</div>
{/each}
