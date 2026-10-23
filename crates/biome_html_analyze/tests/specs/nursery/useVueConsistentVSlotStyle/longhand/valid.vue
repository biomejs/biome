<!-- should not generate diagnostics -->
<template>
  <MyComponent v-slot:default="data">
    <template v-slot:default></template>
    <template v-slot:foo></template>
    <template v-slot:[default]></template>
  </MyComponent>
</template>
