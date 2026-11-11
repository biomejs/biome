<script setup>
/* should not generate diagnostics */
import { defineProps } from "./helpers";

function useSlots() {
  return {};
}

const definitions = defineProps();
const mySlots = useSlots();
</script>
