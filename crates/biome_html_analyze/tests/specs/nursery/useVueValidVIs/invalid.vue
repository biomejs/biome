<!-- should generate diagnostics -->
<template>
  <div v-is:a="foo" />
  <div v-is.a="foo" />
  <div v-is.a.b="foo" />
  <div v-is />
  <div v-is="" />
  <div v-is="   " />
  <MyComponent v-is="foo" />
  <my-component v-is="foo" />
  <component v-is="foo" />
  <Foo.Bar v-is="foo" />
  <MyComponent v-is:a.b />
</template>
