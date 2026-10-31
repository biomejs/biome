<!-- should generate diagnostics -->
<script lang="ts">
	let a = $state(1);
	const typed = $derived.by<number>(() => a * 2);
	const asserted = $derived.by(() => a as number);
</script>
