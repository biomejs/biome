<!-- should generate diagnostics -->

{#each boxes as box}
	{@const volume = box.width * box.height * box.depth}
	<p>{volume}</p>
{/each}
