<!-- should not generate diagnostics -->
{#snippet user(name, age)}
  <p>{name} is {age} years old</p>
{/snippet}

{@render user("John", 42)}
