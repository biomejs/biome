<!-- should not generate diagnostics -->
<script setup lang="ts">
import type { Props } from "./types";

let { count } = defineProps<Props>();

function reset() {
  count = 0;
  count++;
}
</script>
