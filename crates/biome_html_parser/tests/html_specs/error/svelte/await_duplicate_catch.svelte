{#await promise}
  <p>Loading...</p>
{:then value}
  <p>{value}</p>
{:catch error}
  <p>{error.message}</p>
{:catch anotherError}
  <p>{anotherError.message}</p>
{/await}
