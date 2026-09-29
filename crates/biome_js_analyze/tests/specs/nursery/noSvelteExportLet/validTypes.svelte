<!-- should not generate diagnostics -->
<script lang="ts">
	type Props = { name: string };
	export type { Props };

	let { name }: Props = $props();
</script>

<p>{name}</p>
