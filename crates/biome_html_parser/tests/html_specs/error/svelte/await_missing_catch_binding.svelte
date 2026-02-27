{#await promise}
  <p>Loading...</p>
{:then value}
  <p>{value}</p>
{:catch}
  <p>Error occurred</p>
{/await}
