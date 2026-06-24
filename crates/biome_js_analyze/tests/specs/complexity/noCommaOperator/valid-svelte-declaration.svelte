<!-- should not generate diagnostics -->
<script>
	let items = [{ w: 1, h: 2 }];
	const obj = { x: 1, y: 2 };
	const arr = [1, 2];
</script>

{const a = 1, b = 2, c = 3}
{let x = 1, y = 2}
{const { x: objX, y: objY } = obj}
{let [first, second] = arr}

{#each items as item}
	{const width = item.w, height = item.h}
	<span>{width} {height}</span>
{/each}
