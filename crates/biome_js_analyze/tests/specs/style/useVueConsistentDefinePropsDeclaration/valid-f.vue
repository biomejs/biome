<!-- should not generate diagnostics -->
<script setup lang="ts">
import { defineProps } from "./custom";

const props = defineProps({
	kind: { type: String },
});
</script>
