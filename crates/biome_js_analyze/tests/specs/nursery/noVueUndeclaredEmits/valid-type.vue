<!-- should not generate diagnostics -->
<template>
  <button @click="emit('submit')">Submit</button>
</template>

<script setup lang="ts">
const emit = defineEmits<{
  (e: "change", value: string): void;
  (e: "submit" | "reset"): void;
  close: [];
}>();

emit("change", "");
emit("reset");
emit("close");
</script>
