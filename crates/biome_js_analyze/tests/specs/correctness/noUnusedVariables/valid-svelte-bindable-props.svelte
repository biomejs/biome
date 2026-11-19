<!-- should not generate diagnostics -->
<!-- Regression test: $bindable() props that are only written to in the script should  -->
<!-- not be flagged as unused. In Svelte 5, assigning to a $bindable() prop reflects  -->
<!-- the value back to the parent component, so write-only usage is intentional.       -->
<script>
	let {
		backButton = $bindable(),
		nextButton = $bindable(),
	} = $props();

	// backButton is only written to — intentional for a $bindable() prop.
	backButton = { label: 'Back', onClick: () => {} };
	backButton = { label: 'Cancel', onClick: () => {} };
</script>

{#if nextButton}
	<button onclick={nextButton.onClick}>{nextButton.label ?? 'Next'}</button>
{/if}
