<!-- should generate diagnostics -->

<script module>
	export const RADIO = "radio";
</script>

<script>
	const kind = "checkbox";
	const type = "checkbox";
	const radio = RADIO;
	let checked = $state(false);
</script>

<input type={"checkbox"} bind:value={checked} />
<input type={kind} bind:value={checked} />
<input {type} bind:value={checked} />
<input type={`check${"box"}`} bind:value={checked} />
<input type={radio} bind:value={checked} />
{#if checked}
	{@const local = "radio"}
	<input type={local} bind:value={checked} />
{/if}
