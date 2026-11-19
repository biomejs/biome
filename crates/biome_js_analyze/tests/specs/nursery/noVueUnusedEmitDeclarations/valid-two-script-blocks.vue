<!-- should not generate diagnostics -->
<script>
export default {
  emits: ["foo", "bar"],
};
</script>

<script setup>
const emit = defineEmits(["baz"]);
emit("foo");
emit("baz");
</script>

<template>
  <button @click="$emit('bar')">Bar</button>
</template>
