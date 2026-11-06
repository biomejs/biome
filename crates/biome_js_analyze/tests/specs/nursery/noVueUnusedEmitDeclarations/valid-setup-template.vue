<!-- should not generate diagnostics -->
<template>
  <button @click="emits('bar')">Bar</button>
  <button @click="() => emits('baz', 1)">Baz</button>
</template>

<script setup>
const emits = defineEmits(["bar", "baz"]);
</script>
