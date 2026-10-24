<!-- should generate diagnostics -->
<template>
  <div v-for="item in items">
    <MyList v-slot="{ item }">{{ item }}</MyList>
    <MyTable>
      <template #row="{ item }">{{ item }}</template>
    </MyTable>
  </div>
  <MyList v-slot="{ selected }">
    <span v-for="selected in options">{{ selected }}</span>
  </MyList>
  <MyList v-slot="{ entry: current }">{{ current }}</MyList>
</template>

<script setup>
const current = 1;
</script>
