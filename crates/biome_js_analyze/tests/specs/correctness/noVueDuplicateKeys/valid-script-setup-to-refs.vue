<!-- should not emit diagnostics -->

<script setup>
import { toRefs } from 'vue';
const props = defineProps<{ toast: string; number?: number }>()
const { toast } = toRefs(props)
</script>
