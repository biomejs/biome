<!-- should not generate diagnostics -->
<script setup lang="ts">
const a = ref('')
</script>
<template>
  <MyComponent v-model="a as string"></MyComponent>
  <MyComponent v-model="a!"></MyComponent>
  <MyComponent v-model="a as unknown as string"></MyComponent>
  <MyComponent v-model="a!!!!"></MyComponent>
  <MyComponent v-model="(((a!) as unknown)! as string)!"></MyComponent>
</template>
