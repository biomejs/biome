<!-- should generate diagnostics -->
<script setup>
function save(draft = false) {}
</script>
<template>
  <button @click="save()" />
  <button @click="(event) => save(event)" />
</template>
