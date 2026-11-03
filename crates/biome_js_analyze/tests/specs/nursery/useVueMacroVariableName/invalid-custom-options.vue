<script setup>
/* should generate diagnostics */
import { useAttrs, useSlots } from "vue";

const props = defineProps(["foo"]);
const emit = defineEmits(["baz"]);
const slots = defineSlots();
const useSlotsResult = useSlots();
const attrs = useAttrs();
</script>
