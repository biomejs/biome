<div>
	{#if a}
		<span>a</span>
		<!-- before else if -->
	{:else if b}
		<span>b</span>
		<!-- before else -->
	{:else}
		<span>c</span>
		<!-- before /if -->
	{/if}
	{#each items as item}
		<span>{item}</span>
		<!-- before /each -->
	{/each}
	{#if a}
		<!-- only child -->
	{/if}
	{#await p}
		<span>l</span>
		<!-- before then -->
	{:then v}
		<span>{v}</span>
		<!-- before /await -->
	{/await}
	{#key k}
		<span>k</span>
		<!-- before /key -->
	{/key}
	{#snippet s()}
		<span>s</span>
		<!-- before /snippet -->
	{/snippet}
</div>

<div>
	{#each items as item}
		<span>{item}</span>
		<!-- before each else -->
	{:else}
		<span>empty</span>
		<!-- before /each after else -->
	{/each}
	{#if a}<!-- empty same line -->{/if}
	{#if a}
		<span>a</span>
		<!-- first -->

		<!-- second -->
	{/if}
</div>

<div>
	{#if a}
		<span>a</span> <!-- eol -->
	{/if}
	{#if a}
		{#if b}<span>b</span>{/if}<!-- after nested block -->
	{/if}
</div>
