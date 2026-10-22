<!-- should not generate diagnostics -->
<template>
  <p>{{ double }}</p>
  <p>{{ formatter(double) }}</p>
  <p>{{ fromRef() }}</p>
</template>

<script setup>
import { computed, ref } from "vue";

const count = ref(1);
const double = computed(() => count.value * 2);
const formatter = computed(() => (value) => `${value}`);
const fromRef = computed(() => count.value);
</script>
