<!-- should generate diagnostics -->
<script lang="ts">
	import { foo } from "./foo";
	import type { Bar } from "./bar";
	import ValidSvelte from "./validSvelte";

	const lazy = import("./foo");
</script>

<ValidSvelte />
