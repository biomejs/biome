<!-- should not generate diagnostics -->

<script>
	let value = "";
	const getToggled = () => true;
	const setToggled = (value) => value;
	const a = 1;
	const b = 2;
	const set = (value) => value;
</script>

<Toggle bind:pressed={getToggled, setToggled} />
<input bind:value={() => value, (v) => value = v.toLowerCase()} />
<input bind:value={() => [a, b], (v) => set(v)} />
