<!-- should generate diagnostics -->
<template>
  <MyComponent>
    <template #default>content</template>
    <template v-slot:default="{ item }">{{ item }}</template>
  </MyComponent>
</template>
