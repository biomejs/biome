<!-- should generate diagnostics -->
{#snippet render(used, unused)}
	{const value = used}
	{value}
{/snippet}

{@render render(1, 2)}
