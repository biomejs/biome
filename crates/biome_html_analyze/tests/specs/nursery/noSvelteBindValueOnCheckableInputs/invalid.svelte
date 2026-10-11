<!-- should generate diagnostics -->

<script>
	let checked = $state(false);
	let selected = $state("a");
	let value = $state(false);
</script>

<input type="checkbox" bind:value={checked} />
<input type="radio" bind:value={selected} />
<input type="CheckBox" bind:value={checked} />
<input type='radio' bind:value={selected}>
<input type=checkbox bind:value={checked} />
<input bind:value={checked} type="checkbox" />
<input type="checkbox" bind:value />
<input type="radio" bind:value />
<input type="checkbox" bind:value={() => checked, (v) => (checked = v)} />
<input type="checkbox" checked bind:value={checked} />
<input type="checkbox" {checked} bind:value={selected} />
<input type="checkbox" bind:value={selected} bind:checked={checked} />
<input type="radio" bind:group={selected} bind:value={checked} />
<input type="radio" bind:value={() => selected, (v) => (selected = v)} />
