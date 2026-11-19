<!-- should not generate diagnostics -->
<script module>
	export let count = 0;

	let total = 0;
	export { total };
</script>

<script>
	let { name } = $props();
</script>

<p>{name} {count} {total}</p>
