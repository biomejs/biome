<!-- should not generate diagnostics -->
<script lang="ts">
// this makes it typescript
</script>

<div
	onclick={(e) => {
		if ((e.target as HTMLElement).closest("button")) {
			return;
		}
		e.currentTarget.parentElement?.querySelector("input")?.focus();
	}}
>
</div>
