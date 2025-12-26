{#await promise}
<div class="loading">
<span class="spinner"></span>
<p>Please wait while we load your data...</p>
</div>
{:then data}
<div class="success">
<h2>Success!</h2>
<pre>{JSON.stringify(data, null, 2)}</pre>
</div>
{:catch error}
<div class="error">
<h2>Error</h2>
<p>{error.message}</p>
<button on:click={retry}>Retry</button>
</div>
{/await}
