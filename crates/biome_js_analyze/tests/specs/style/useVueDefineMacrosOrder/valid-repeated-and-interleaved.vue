<!-- should not generate diagnostics -->
<script setup lang="ts">
import { ref } from 'vue';
const first = defineModel('first');
defineModel('second');
interface Props { value: string }
defineProps<Props>();
type Events = { update: [] };
const emit = defineEmits<Events>();
debugger;
const count = ref(0);
</script>
