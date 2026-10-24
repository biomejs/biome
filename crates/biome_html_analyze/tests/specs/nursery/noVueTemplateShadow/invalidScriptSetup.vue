<!-- should generate diagnostics -->
<template>
  <div v-for="count in 5"></div>
  <div v-for="helper in 5"></div>
  <div v-for="size in 5"></div>
  <div v-for="props in 5"></div>
  <div v-for="modelValue in 5"></div>
</template>

<script setup>
import { helper } from "./helper.js";

let count = 42;
const props = defineProps({ size: Number });
const value = defineModel();
</script>
