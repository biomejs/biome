<!-- should not generate diagnostics -->
<script setup>
await doSomething();
</script>
<script>
import { watchEffect } from "vue";

await doSomething();
watchEffect(() => {});
</script>
