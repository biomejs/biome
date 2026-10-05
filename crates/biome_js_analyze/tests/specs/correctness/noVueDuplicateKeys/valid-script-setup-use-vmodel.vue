<!-- should not generate diagnostics -->
<script setup lang="ts">
import { useVModel } from '@vueuse/core';
const props = defineProps<{ modelValue?: string | number }>();
const modelValue = useVModel(props, 'modelValue');
</script>
