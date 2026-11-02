<!-- should generate diagnostics -->
<template>
  <div v-for="i in 5">
    <div v-for="i in 5"></div>
  </div>
  <ul v-for="(item, index) in items">
    <li v-for="(child, index) in item.children">
      <span v-for="{ item } in child.entries">{{ item }}</span>
    </li>
  </ul>
  <div v-for="(value, key, position) in record">
    <div v-for="[key, position] in pairs"></div>
  </div>
  <div v-for="(row, row) in rows"></div>
</template>
