<!-- should not generate diagnostics -->
<script setup>
import { watch } from "vue";

watch(source, () => {});

await doSomething();

watch(source, () => {});
</script>
