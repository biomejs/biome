<!-- should generate diagnostics -->
<template>
  <MyComponent v-bind.sync="bar" />
  <MyComponent :foo.sync.unknown="bar" />
  <MyComponent :[dynamicArg].sync.unknown="bar" />
  <MyComponent :foo.camel.sync="bar" />
  <MyComponent :foo.sync />
</template>
