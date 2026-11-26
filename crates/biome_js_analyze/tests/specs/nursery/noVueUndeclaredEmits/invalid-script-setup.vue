<!-- should generate diagnostics -->
<template>
  <button @click="emit('open')">Open</button>
  <button @click="$emit('toggle')">Toggle</button>
</template>

<script setup>
const emit = defineEmits(["change"]);

function close() {
  emit("close");
}
</script>
