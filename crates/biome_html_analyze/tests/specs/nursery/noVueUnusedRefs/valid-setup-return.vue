<!-- should not generate diagnostics -->
<script>
import { ref } from "vue";

export default {
  setup() {
    const x = ref(null);
    return { x };
  },
};
</script>

<template>
  <input ref="x" />
</template>
