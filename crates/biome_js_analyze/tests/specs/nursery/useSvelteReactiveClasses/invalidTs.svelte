<!-- should generate diagnostics -->
<script lang="ts">
	const wrapped = (new Date() as Date);
	wrapped.setTime(0);

	const map = new Map<string, number>()!;
	map.set("one", 1);

	const set = $state(new Set<number>()) satisfies Set<number>;
	set.add(1);
</script>
