<!-- should not generate diagnostics -->

<script setup lang="ts">
import { toRefs } from 'vue';
const props = defineProps<{ toast: string; number?: number }>()
const { toast } = toRefs(props)
</script>
