<!-- Await block with comments -->
{#await promise}
  <!-- Loading state -->
  <p>Loading...</p>
{:then value}
  <!-- Success state -->
  <p>Value: {value}</p>
{:catch error}
  <!-- Error state -->
  <p>Error: {error.message}</p>
{/await}
