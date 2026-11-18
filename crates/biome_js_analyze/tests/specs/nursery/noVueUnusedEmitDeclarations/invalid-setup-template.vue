<!-- should generate diagnostics -->
<template>
  <button @click="emit('bar')">Bar</button>
</template>

<script setup>
const emit = defineEmits(["foo", "bar"]);
</script>
