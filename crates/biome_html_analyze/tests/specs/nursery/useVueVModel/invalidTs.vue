<!-- should generate diagnostics -->
<script setup lang="ts">
import { ref } from 'vue'
const title = ref('')
</script>

<template>
  <MyComponent :title="title" @update:title="(value: string) => title = value" />
  <MyComponent :title="title" @update:title="(value?: string) => title = value" />
</template>
