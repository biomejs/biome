<!-- should not generate diagnostics -->
<script setup lang="ts">
import { computed } from "vue";
import { z } from "zod";

const numberWithCatch = computed(() => z.number().catch(42));
const chained = computed(() => z.catch(z.string(), "default").then((value) => value).finally(() => {}));
const deep = computed(() => z.a?.["b"].c!.d.method().catch((error) => error).finally(() => {}));
</script>
<script>
export default {
	computed: {
		foo() {
			return z.object({}).shape.catch(0);
		},
	},
};
</script>
