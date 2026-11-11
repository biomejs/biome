<!-- should not generate diagnostics -->
<script>
import { reactive, toRefs } from "vue";

export default {
  setup() {
    const state = reactive({ a: 1, b: 2 });
    return { ...toRefs(state) };
  },
};
</script>

<template>
  <p>{{ a }} {{ b }} {{ c }}</p>
</template>
