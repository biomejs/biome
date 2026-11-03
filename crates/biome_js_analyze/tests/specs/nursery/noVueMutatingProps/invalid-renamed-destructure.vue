<!-- should generate diagnostics -->
<script setup>
const { modelValue: items } = defineProps({
  modelValue: Array,
});

function add() {
  items.push(1);
}
</script>

<template>
  <button @click="items.pop()">Remove</button>
</template>
