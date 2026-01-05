{#await promise}
  <p>Loading...</p>
{:catch error}
  <p>{error.message}</p>
{:then value}
  <p>{value}</p>
{/await}
