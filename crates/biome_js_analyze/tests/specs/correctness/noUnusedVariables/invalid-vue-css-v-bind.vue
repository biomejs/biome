<!-- should generate diagnostics -->
<script setup lang="ts">
import { computed } from "vue";

const size = computed(() => "1px");
const unused = computed(() => "2px");
</script>

<template>
  <button>hello</button>
</template>

<style scoped>
button {
  height: v-bind(size);
}
</style>
