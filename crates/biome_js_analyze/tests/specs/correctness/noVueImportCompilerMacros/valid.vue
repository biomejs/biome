<!-- should not generate diagnostics -->
<script setup>
import { ref } from "vue";

const props = defineProps({});
const count = ref(0);
</script>

<template>
  <div>{{ count }}</div>
</template>
