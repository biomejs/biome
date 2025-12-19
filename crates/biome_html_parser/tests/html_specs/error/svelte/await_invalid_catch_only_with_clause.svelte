{#await promise catch error}
  <p>{error.message}</p>
{:catch anotherError}
  <p>{anotherError.message}</p>
{/await}
