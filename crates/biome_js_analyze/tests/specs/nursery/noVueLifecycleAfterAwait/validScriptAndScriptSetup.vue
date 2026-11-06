<!-- should not generate diagnostics -->
<script setup>
await doSomething();
</script>
<script>
import { onMounted } from "vue";

await doSomething();
onMounted(() => {});
</script>
