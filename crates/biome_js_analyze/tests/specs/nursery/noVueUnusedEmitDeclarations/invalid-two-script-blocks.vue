<!-- should generate diagnostics -->
<script>
export default {
  emits: ["foo", "bar"],
};
</script>

<script setup>
const count = 1;
</script>

<template>
  <button @click="$emit('bar')">{{ count }}</button>
</template>
