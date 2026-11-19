<!-- should not generate diagnostics -->
<template>
  <MyComponent v-slot="data">
    <template v-slot></template>
    <template #foo></template>
  </MyComponent>
</template>
