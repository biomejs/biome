<!-- should generate diagnostics -->
<template>
  <MyComponent v-bind:foo.sync="bar" />
  <MyComponent :foo.sync="bar" />
  <MyComponent v-bind:[dynamicArg].sync="bar" />
  <MyComponent :[dynamicArg].sync="bar" />
  <MyComponent v-bind:foo.sync="x.foo" class="a" />
  <MyComponent
    :foo.sync="foo[x - 1]"
  />
</template>
