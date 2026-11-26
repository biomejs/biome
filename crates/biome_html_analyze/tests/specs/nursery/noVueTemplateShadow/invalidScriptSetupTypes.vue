<!-- should generate diagnostics -->
<template>
  <div v-for="title in 5"></div>
  <div v-for="item in 5"></div>
</template>

<script setup lang="ts">
defineProps<{ title: string }>();
const item = "selected";
</script>
