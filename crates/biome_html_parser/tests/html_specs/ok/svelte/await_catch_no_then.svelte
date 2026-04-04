{#await finixInit}
	<Icons.DotsBounce />
{:catch error}
	<p>Could not initialize form: {error.message}</p>
{/await}
