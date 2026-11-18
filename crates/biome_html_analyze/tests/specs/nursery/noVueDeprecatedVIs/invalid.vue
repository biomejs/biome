<!-- should generate diagnostics -->
<template>
  <div v-is="'MyComponent'"></div>
  <div v-is="'MyComponent'" />
  <tr v-is="'my-row'"></tr>
  <div v-is="componentName"></div>
  <div v-is></div>
  <div v-is:arg="'MyComponent'"></div>
  <div v-is.modifier="'MyComponent'"></div>
  <ul>
    <li v-for="item in items" :key="item.id" v-is="'MyItem'"></li>
  </ul>
</template>
