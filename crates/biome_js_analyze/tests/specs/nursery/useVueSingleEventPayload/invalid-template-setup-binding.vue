<!-- should generate diagnostics -->
<template>
  <button @click="emit('change', 'foo', 'bar')">Change</button>
</template>

<script setup lang="ts">
const emit = defineEmits<{
  (e: "change", a: string, b: string): void;
}>();
</script>
