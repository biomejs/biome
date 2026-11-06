<!-- should not generate diagnostics -->
<script setup>
const items = [{ id: 1, name: "alpha" }];
</script>

<template>
  <ul>
    <li v-for="(item, index) in items" :key="item.id">
      {{ index }} {{ item.name }} {{ Math.max(index, 1) }} {{ undefined }}
    </li>
  </ul>
  <List v-slot="{ row }">{{ row.label }}</List>
  <input @input="console.log($event)" />
  <p>{{ $slots.default }} {{ $attrs.class }} {{ $refs.input }} {{ $el }}</p>
</template>
