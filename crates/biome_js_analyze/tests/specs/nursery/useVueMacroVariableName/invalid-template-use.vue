<script setup>
/* should generate diagnostics */
const emits = defineEmits(["change"]);
</script>

<template>
  <button @click="emits('change')">Change</button>
</template>
