<!-- should generate diagnostics -->
<template>
  <div ref="root" :data-a="A" />
</template>

<script>
const A = "foo";
</script>

<script setup>
import { ref } from "vue";
const root = ref();
</script>
