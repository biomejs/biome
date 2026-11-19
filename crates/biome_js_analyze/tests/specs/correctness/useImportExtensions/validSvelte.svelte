<!-- should not generate diagnostics -->
<script lang="ts">
	import { foo } from "./foo.ts";
	import type { Bar } from "./bar.ts";
	import InvalidVue from "./invalidVue.vue";
	import "biome";
</script>

{foo}
