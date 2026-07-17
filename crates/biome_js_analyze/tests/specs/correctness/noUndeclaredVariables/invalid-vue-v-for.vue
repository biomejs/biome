<!-- should generate diagnostics -->
<script setup>
const rows = [{ id: 1, meta: { label: "alpha" } }];
</script>

<template>
  <p v-for="item in missingItems" :key="item.id">{{ item.name }}</p>
  <p v-for="({ id, meta: { label } }, index) in rows" :key="id">
    {{ label }} {{ index }} {{ missingLabel }}
  </p>
</template>
