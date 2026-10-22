<!-- should generate diagnostics -->
<template>
  <p>{{ double() }}</p>
  <p>{{ label() }}</p>
</template>

<script setup>
import { computed } from "vue";

const double = computed(() => 2);
const label = computed({
  get: () => "label",
  set: (value) => {},
});
</script>
