<!-- should generate diagnostics -->
<script setup>
const items = [{ id: 1, name: "alpha" }];
const rows = [{ id: 1, meta: { label: "alpha" } }];

function unusedHelper() {
  return "not rendered";
}
</script>

<template>
  <p v-for="item in items" :key="item.id">{{ item.name }}</p>
  <p v-for="({ id, meta: { label } }, index) in rows" :key="id">
    {{ label }} {{ index }}
  </p>
</template>
