<!-- should generate diagnostics -->
<template>
  <div v-text="foobar"></div>
  <span v-text="message" />
  <p v-text="'literal'">ignored content</p>
  <MyComponent v-text="label" />
  <div v-text></div>
  <div v-text:arg="foobar"></div>
  <div v-text.modifier="foobar"></div>
  <ul>
    <li v-for="item in items" :key="item.id" v-text="item.name"></li>
  </ul>
</template>
