<!-- should generate diagnostics -->
<template>
  <MyComponent v-slot="data">{{ data }}</MyComponent>
  <MyComponent v-slot:default="data">{{ data }}</MyComponent>
  <MyComponent>
    <template v-slot>content</template>
    <template v-slot:foo>content</template>
  </MyComponent>
</template>
