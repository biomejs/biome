<!-- should not generate diagnostics -->
<script setup>
const items = [{ id: 1, name: "alpha" }];
const keyedItems = { a: { name: "alpha" } };
const rows = [{ id: 1, meta: { label: "alpha" }, extra: true }];
const pairs = [["a", "b"]];
</script>

<template>
  <p v-for="item in items" :key="item.id">{{ item.name }}</p>
  <p v-for="(value, key, index) of keyedItems" :key="key">{{ value.name }} {{ index }}</p>
  <p v-for="({ id, meta: { label }, ...rest }, idx) in rows" :key="id">
    {{ label }} {{ rest.extra }} {{ idx }}
  </p>
  <p v-for="([first, , ...tail]) in pairs" :key="first">{{ tail }}</p>
</template>
