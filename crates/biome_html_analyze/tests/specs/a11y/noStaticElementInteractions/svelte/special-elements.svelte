<!-- should not generate diagnostics -->
<!-- Svelte special elements are not real DOM elements. -->
<svelte:window
	onkeydown={(e) => {
		if (e.key === ' ') {
			e.preventDefault()
		}
	}}
/>
<svelte:document onmousemove={() => void 0} />
<svelte:body onmouseenter={() => void 0} />
