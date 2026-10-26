<script setup>
const rows = [{ id: 1 }];
</script>

<template>
  <p v-for="{ id, label = missingLabel } in rows" :key="id">{{ label }}</p>
</template>
