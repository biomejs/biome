<!-- should generate diagnostics -->

<script>
	let checked = $state(false);
</script>

<input TYPE="checkbox" bind:value={checked} />
<!-- `CHECKED` is the `checked` attribute, so renaming the binding would repeat it: no fix. -->
<input Type="checkbox" CHECKED bind:value={checked} />
