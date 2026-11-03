<!-- should generate diagnostics -->
<script setup lang="ts">
const a = () => 1
</script>
<template>
  <MyComponent v-model="a() as string"></MyComponent>
  <MyComponent v-model="a()!"></MyComponent>
  <MyComponent v-model="a() as unknown as string"></MyComponent>
  <MyComponent v-model="a()!!!!"></MyComponent>
  <MyComponent v-model="(((a()!) as unknown)! as string)!"></MyComponent>
  <MyComponent v-model="a?.b as string"></MyComponent>
</template>
