<!-- should not generate diagnostics -->
<script setup lang="ts">
import { SKIP_CATEGORIES as skipCategories } from "ott-common";
</script>
<template>
  <div :data-categories="skipCategories"></div>
</template>
