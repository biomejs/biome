<!-- should not generate diagnostics -->
{#if true}
	{const $derived = { by: (callback) => callback() }}
	{$derived.by(() => 1)}
{/if}
