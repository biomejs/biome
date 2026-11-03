<!-- should not generate diagnostics -->
<template>
  <button @click="emit('close')">Close</button>
  <button @click="$emit('change')">Change</button>
  <input @input="$emit('update:modelValue', $event)" />
</template>

<script setup>
const model = defineModel();
const emit = defineEmits(["change", "close"]);

function close() {
  emit("close");
  emit("update:modelValue", "");
}
</script>
