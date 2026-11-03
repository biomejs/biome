<!-- should not generate diagnostics -->
<script setup>
import { ref } from "vue";

const { count, items } = defineProps({
  count: Number,
  items: Array,
});

const local = ref(items.slice());
local.value.push(count);
</script>

<template>
  <span>{{ count }} {{ items.length }}</span>
</template>
