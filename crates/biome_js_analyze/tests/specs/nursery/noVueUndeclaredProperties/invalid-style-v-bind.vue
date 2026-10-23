<!-- should generate diagnostics -->
<script setup>
const color = "red";
</script>

<template>
  <p>Text</p>
</template>

<style>
p {
  color: v-bind(color);
  background: v-bind(background);
}
</style>
