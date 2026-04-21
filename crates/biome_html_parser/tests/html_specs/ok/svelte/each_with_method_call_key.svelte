{#each numbers as number, index (number.toString())}
  <p>{number}</p>
{/each}

{#each numbers as number (key(number))}
  <p>{number}</p>
{/each}
