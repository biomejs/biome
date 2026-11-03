<!-- should not generate diagnostics -->
<template>
  <div v-is="foo" />
  <div v-is="'foo'" />
  <tr v-is="'my-row'"></tr>
  <div v-bind:foo="foo" />
  <svg><g v-is="'my-group'" /></svg>
  <math><mi v-is="'my-identifier'" /></math>
  <MyComponent :is="foo" />
  <component :is="foo" />
</template>
