<!-- should not generate diagnostics -->
<script setup lang="ts" generic="T">
import { ref } from "vue";

const value = ref("");
const items = ref<T[]>([]);
</script>

<template>
  <input @input="value = ($event.target as HTMLInputElement).value" />
  <input @keydown="(e: KeyboardEvent) => console.log(e.key)" />
  <p v-for="item in items">{{ item as T }}</p>
</template>
