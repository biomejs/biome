<!-- should generate diagnostics -->
<script setup lang="ts">
function save(event: Event) {}
</script>
<template>
  <button @click="(event: Event) => save(event)" />
  <button @click="save(undefined as never)" />
</template>
