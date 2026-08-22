<!-- should not generate diagnostics -->
<script setup lang="ts">
import { computed } from "vue";

const size = computed(() => "1px");
const theme = { color: "red" };
const gap = computed(() => "4px");
</script>

<template>
  <button>hello</button>
</template>

<style scoped>
button {
  height: v-bind(size);
  color: v-bind("theme.color");
  margin: v-bind(gap);
}
</style>
