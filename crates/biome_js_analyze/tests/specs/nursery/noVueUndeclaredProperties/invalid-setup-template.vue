<!-- should generate diagnostics -->
<script setup>
const props = defineProps(["name", "def"]);
let count = 0;
</script>

<template>
  <div>{{ name }}: {{ count }}</div>
  <div>{{ label }}: {{ cnt }}</div>
</template>
