<!-- should not generate diagnostics -->
<CollapsiblePrimitive.Content>
	{#snippet child({ props, open })}
		{#if open}
			<div {...props} transition:slide={{ duration }}>
				{@render children?.()}
			</div>
		{/if}
	{/snippet}
</CollapsiblePrimitive.Content>
