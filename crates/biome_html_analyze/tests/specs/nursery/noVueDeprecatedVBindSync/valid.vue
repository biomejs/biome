<!-- should not generate diagnostics -->
<template>
  <MyComponent v-bind:foo="bar" />
  <MyComponent :foo="bar" />
  <MyComponent v-bind:[dynamicArg]="bar" />
  <MyComponent :[dynamicArg]="bar" />
  <MyComponent v-model:foo="bar" />
  <MyComponent v-bind="props" />
  <MyComponent :foo.camel="bar" />
  <MyComponent v-on:foo.sync="bar" />
  <MyComponent @update.sync="bar" />
  <MyComponent v-model.sync="bar" />
  <MyComponent :foo.syncing="bar" />
</template>
