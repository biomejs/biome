{#each activeFilters as { id, component: Filter } (id)}
  <Filter />
{/each}

{#each list as { foo: bar, baz: qux }}
  <p>{bar} {qux}</p>
{/each}
