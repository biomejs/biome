{#each activeFilters as { id, component: Filter } (id)}
  <Filter />
{/each}

{#each list as { foo: bar, baz: qux }}
  <p>{bar} {qux}</p>
{/each}

{#each list as { nested: { a, b } }}
  <p>{a} {b}</p>
{/each}

{#each list as { arr: [x, y] }}
  <p>{x} {y}</p>
{/each}
