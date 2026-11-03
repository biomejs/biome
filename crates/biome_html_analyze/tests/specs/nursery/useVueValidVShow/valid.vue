<!-- should not generate diagnostics -->

<template>
  <div v-show="foo"></div>
  <div v-show="a && b"></div>
  <div v-show="isVisible" v-if="ready"></div>
  <MyComponent v-show="foo" />
  <input v-show="foo" />
  <template v-if="foo">
    <div v-show="bar"></div>
  </template>
  <div v-if="foo"></div>
  <div v-html="content"></div>
  <div v-showing:aaa></div>
</template>
