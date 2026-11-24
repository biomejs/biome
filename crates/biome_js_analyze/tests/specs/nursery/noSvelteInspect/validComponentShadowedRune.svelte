<!-- should not generate diagnostics -->
{#if true}
	{const $inspect = { trace() {} }}
	{$inspect.trace()}
{/if}
