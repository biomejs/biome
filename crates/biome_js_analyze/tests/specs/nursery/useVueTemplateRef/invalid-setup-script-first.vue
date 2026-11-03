<!-- should generate diagnostics -->
<template>
  <div ref="root" :data-a="A" />
</template>

<script setup>
import { ref } from "vue";
const root = ref();
</script>

<script>
const A = "foo";
</script>
