<!-- should not generate diagnostics -->
<script setup lang="ts">
import type { Props } from "./types";

const props = defineProps<Props>();
console.log(props.anything);
</script>

<template>
  <p>{{ foo }} {{ unknown }}</p>
</template>
