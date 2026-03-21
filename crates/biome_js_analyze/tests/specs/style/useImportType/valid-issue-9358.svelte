<!-- should not generate diagnostics -->
<script lang="ts">
	import { buttonClass } from "./buttonClass.ts";
	export let variant: Parameters<typeof buttonClass>[0] = "default";
</script>

<button class={buttonClass(variant)}>Test</button>
