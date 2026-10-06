{#snippet children(item)}
  {item}
{/snippet}

{#snippet children(item, foo, bar)}
  {item}
{/snippet}

{#snippet row({ id, name, description }, index, firstVeryLongParameterName, secondVeryLongParameterName)}
  {name}
{/snippet}
