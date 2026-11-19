<!-- should not generate diagnostics -->
<script setup lang="ts">
import type { PropType } from "vue";

type Size = "small" | "large";
const sizes = ["small", "large"];

const props = defineProps({
  size: String as PropType<Size>,
  sizes: Array as PropType<typeof sizes>,
});
</script>
