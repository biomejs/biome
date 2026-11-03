<script setup lang="ts">
/* should generate diagnostics */
import { useSlots } from "vue";

const typedSlots = defineSlots<{ default(): any }>();
const componentSlots = useSlots();
</script>
