<!-- should generate diagnostics -->
<template>
  <MyComponent #default="data">{{ data }}</MyComponent>
  <MyComponent v-slot:default="data">{{ data }}</MyComponent>
  <MyComponent #default="data" />
  <MyComponent>
    <template v-slot>content</template>
    <template v-slot:default>content</template>
    <template v-slot:foo>content</template>
    <template v-slot:[foo]>content</template>
    <template v-slot:header="{ title }">{{ title }}</template>
    <template v-slot:foo.bar>content</template>
  </MyComponent>
</template>
