<!-- should not generate diagnostics -->
<script setup>
import { onMounted } from "vue";

await doSomething();

onMounted(() => {});
</script>
