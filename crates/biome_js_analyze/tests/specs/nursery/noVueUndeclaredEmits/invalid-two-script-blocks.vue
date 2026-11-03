<!-- should generate diagnostics -->
<template>
  <button @click="$emit('close')">Close</button>
</template>

<script>
export default {
  inheritAttrs: false,
};
</script>

<script setup>
defineEmits(["change"]);
</script>
