{#await promise}
<p>Loading...</p>
{:then value}
<p>The value is {value}</p>
{:catch error}
<p>Something went wrong: {error.message}</p>
{/await}
