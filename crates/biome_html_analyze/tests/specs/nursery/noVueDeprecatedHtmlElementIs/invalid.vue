<!-- should generate diagnostics -->
<template>
  <div is="foo" />
  <div :is="foo" />
  <div v-bind:is="foo" />
  <tr is="my-row"></tr>
  <button is="fancy-button">Click</button>
  <div is />
  <div is="" />
  <div is="Vue:foo" />
  <div :is="'vue:foo'" />
  <svg><g is="foo" /></svg>
  <math><mi is="foo" /></math>
  <template is="foo"></template>
</template>
