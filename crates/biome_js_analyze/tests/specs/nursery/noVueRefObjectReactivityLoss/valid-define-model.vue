/* should not generate diagnostics */
<script setup>
import { computed } from 'vue'
const model = defineModel();
const [modelValue, modifiers] = defineModel();
console.log(modifiers.value);
const doubled = computed(() => model.value * 2 + modelValue.value);
</script>
