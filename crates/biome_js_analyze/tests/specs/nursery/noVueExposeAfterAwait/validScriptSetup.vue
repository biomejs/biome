<!-- should not generate diagnostics -->
<script setup>
import { onMounted } from "vue";

defineExpose({});
await doSomething();
onMounted(() => {});
</script>
