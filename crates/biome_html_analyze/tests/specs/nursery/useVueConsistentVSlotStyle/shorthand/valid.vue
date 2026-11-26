<!-- should not generate diagnostics -->
<template>
  <MyComponent #default="data">
    <template #default></template>
    <template #foo></template>
    <template #[foo]></template>
  </MyComponent>
</template>
