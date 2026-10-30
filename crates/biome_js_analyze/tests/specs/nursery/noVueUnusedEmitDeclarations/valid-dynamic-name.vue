<!-- should not generate diagnostics -->
<template>
  <button @click="emit(name)">Send</button>
</template>

<script setup>
const emit = defineEmits(["foo", "bar"]);
const name = "foo";
</script>
