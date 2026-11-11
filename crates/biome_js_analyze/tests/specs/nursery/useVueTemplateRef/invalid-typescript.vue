<!-- should generate diagnostics -->
<template>
  <input ref="input" />
</template>

<script setup lang="ts">
import { ref } from "vue";

const input = ref<HTMLInputElement | null>(null);
</script>
