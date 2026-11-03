<!-- should not generate diagnostics -->
<template>
  <MyComponent v-slot="data">
    <template #default></template>
    <template #foo></template>
  </MyComponent>
  <my-component v-slot>
    {{ value }}
  </my-component>
  <MyComponent>
    <template #[name]></template>
    <template #[default]></template>
    <template #header="{ title }">{{ title }}</template>
  </MyComponent>
  <div v-bind:foo="bar" @click="onClick"></div>
</template>
