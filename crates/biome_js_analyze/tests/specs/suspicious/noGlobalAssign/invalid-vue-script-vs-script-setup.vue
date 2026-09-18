<!-- should generate diagnostics -->
<script setup>
const open = defineModel();
open = true;
</script>

<script>
open = true;
</script>

<template>
  <button @click="open = !open">Swap</button>
</template>
