<!-- should not generate diagnostics -->
<script setup lang="ts">
import { ALL_SKIP_CATEGORIES } from "ott-common";
const model = defineModel<string[]>();
</script>
<template>
  <div :data-categories="ALL_SKIP_CATEGORIES">{{ model }}</div>
</template>
