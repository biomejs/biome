<!-- should not generate diagnostics -->
<script setup>
const open = defineModel();
</script>

<template>
  <button @click="open = !open">Toggle</button>
</template>
