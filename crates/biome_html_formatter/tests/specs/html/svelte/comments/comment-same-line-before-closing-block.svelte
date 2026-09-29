<div>
	{#if a}
		<span>a</span><!-- same line -->{/if}
	{#if a}
		<span>a</span><!-- before else -->{:else}
		<span>b</span><!-- before /if -->{/if}
	{#each items as item}
		<span>{item}</span><!-- before /each -->{/each}
	{#await p}
		<span>l</span><!-- before then -->{:then v}
		<span>{v}</span><!-- before /await -->{/await}
	{#if a}
		text<!-- after text -->{/if}
</div>
