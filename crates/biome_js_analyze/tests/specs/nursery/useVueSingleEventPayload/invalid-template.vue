<!-- should generate diagnostics -->
<template>
  <button @click="$emit('change', value1, value2)">Change</button>
  <button @click="$emit('update', a, b, c)">Update</button>
</template>

<script>
export default {
  emits: ["change", "update"],
};
</script>
