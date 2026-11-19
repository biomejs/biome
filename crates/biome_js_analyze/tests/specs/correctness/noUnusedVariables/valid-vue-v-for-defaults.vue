<!-- should not generate diagnostics -->
<script setup>
const rows = [{ id: 1 }];
const fallbackLabel = "none";
const fallbackSize = () => 1;
</script>

<template>
  <p v-for="{ id, label = fallbackLabel } in rows" :key="id">{{ label }}</p>
  <p v-for="([size = fallbackSize()], index = 0) in rows" :key="index">{{ size }}</p>
</template>
