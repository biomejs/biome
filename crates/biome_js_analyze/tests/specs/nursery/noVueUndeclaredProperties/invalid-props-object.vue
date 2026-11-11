<!-- should generate diagnostics -->
<script setup>
import { watch } from "vue";

const props = defineProps(["name", "def"]);

watch(
  () => props.def,
  () => {},
);
watch(
  () => props.undef,
  () => {},
);
</script>

<template>
  <p>{{ props.name }} {{ props.title }} {{ $props.subtitle }}</p>
</template>
