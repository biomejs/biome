<!-- should not generate diagnostics -->
<template>
  <div v-for="i in 5">
    <div v-for="j in 5"></div>
  </div>
  <div v-for="Item in items">{{ Item }}</div>
</template>

<script setup lang="ts">
type Item = { id: number };
let k = 42;
const items: Item[] = [];
defineProps({ size: Number });
</script>
