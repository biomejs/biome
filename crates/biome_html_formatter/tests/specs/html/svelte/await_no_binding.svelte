{#await promise}
<p>Loading...</p>
{:then}
<p>Done</p>
{:catch}
<p>Something went wrong</p>
{/await}

{#await otherPromise}
<p>Loading...</p>
{:then}
<p>Done</p>
{/await}

{#await thirdPromise}
<p>Loading...</p>
{:catch}
<p>Something went wrong</p>
{/await}

{#await promise then}
<p>Done</p>
{/await}

{#await promise catch}
<p>Failed</p>
{/await}
