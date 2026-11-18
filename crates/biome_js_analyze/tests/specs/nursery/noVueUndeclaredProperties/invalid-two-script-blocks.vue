<!-- should generate diagnostics -->
<script>
export default {
  inheritAttrs: false,
};
</script>

<script setup>
import { ref } from "vue";

const count = ref(0);
</script>

<template>
  <p>{{ count }} {{ total }}</p>
</template>
