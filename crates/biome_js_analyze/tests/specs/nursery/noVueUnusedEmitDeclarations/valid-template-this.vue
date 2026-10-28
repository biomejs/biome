<!-- should not generate diagnostics -->
<template>
  <button @click="this.$emit('bar')">Bar</button>
</template>

<script>
export default {
  emits: ["bar"],
};
</script>
