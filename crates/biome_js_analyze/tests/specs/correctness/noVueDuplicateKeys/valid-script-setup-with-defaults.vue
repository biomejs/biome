<!-- should not generate diagnostics -->
<script setup lang="ts">
import { toRef, toRefs } from "vue";

const props = withDefaults(defineProps<{ text: string; count?: number; label?: string }>(), {
    count: 0,
});
const { text } = toRefs(props);
const count = toRef(props, "count");
const label = props.label;
</script>
