<!-- should not generate diagnostics -->
<template>
  <button @click="$emit('change')">Change</button>
  <button @click="$emit('change', value)">Change</button>
  <button @click="$emit('change', { a, b })">Change</button>
  <button @click="emit('update', { a: 'foo', b: 'bar' })">Update</button>
</template>

<script setup lang="ts">
const emit = defineEmits<{
  (e: "update", payload: { a: string; b: string }): void;
}>();
</script>
