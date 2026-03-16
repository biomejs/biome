<!-- should not generate diagnostics -->
<script lang="ts">
import { Avatar as AvatarPrimitive } from "bits-ui";
import { cn } from "$lib/utils.js";

let {
	ref = $bindable(null),
	class: className,
}: AvatarPrimitive.FallbackProps = $props();
</script>

<!-- used as value here -->
<AvatarPrimitive.Fallback
	bind:ref
	class="something nice"
/>
