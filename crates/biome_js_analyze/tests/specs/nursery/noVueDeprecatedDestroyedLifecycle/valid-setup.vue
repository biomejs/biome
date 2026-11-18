<!-- should not generate diagnostics -->
<script setup>
import { onBeforeUnmount, onUnmounted } from "vue";

onBeforeUnmount(() => {});
onUnmounted(() => {});

const destroyed = {
  beforeDestroy() {},
};
</script>
