<!-- should not generate diagnostics -->
<script setup>
import { toRef } from 'vue';
const props = defineProps(['foo', 'bar'])
const foo = toRef(props, 'foo')
const bar = toRef(props, 'bar')
</script>
