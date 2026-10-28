<!-- should generate diagnostics -->
<template>
  <button @click="$emit('bar')">Bar</button>
</template>

<script>
export default {
  emits: ["foo", "bar"],
};
</script>
