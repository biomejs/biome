<!-- should generate diagnostics -->
<template>
  <button @click="emit('open')">Open</button>
</template>

<script setup lang="ts">
const emit = defineEmits<{
  (e: "change", value: string): void;
  (e: "submit" | "reset"): void;
}>();

emit("close");
</script>
