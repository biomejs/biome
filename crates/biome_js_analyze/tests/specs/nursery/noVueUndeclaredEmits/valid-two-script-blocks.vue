<!-- should not generate diagnostics -->
<template>
  <button @click="$emit('close')">Close</button>
</template>

<script>
export default {
  emits: ["close"],
};
</script>

<script setup>
const label = "Close";
</script>
