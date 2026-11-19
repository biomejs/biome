<!-- should generate diagnostics -->
<template>
  <MyComponent #default="data">{{ data }}</MyComponent>
  <MyComponent v-slot="data">{{ data }}</MyComponent>
  <MyComponent>
    <template #default>content</template>
    <template v-slot>content</template>
    <template #foo>content</template>
    <template #[foo]>content</template>
  </MyComponent>
</template>
