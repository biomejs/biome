<!-- should generate diagnostics -->
<script setup lang="ts">
import type { Props } from "./types";

const props = defineProps<Props>();

function update() {
  props.value = "test";
  props.list.push(1);
}
</script>

<template>
  <input v-model="props.value" />
  <button @click="$props.value = 'test'">Assign</button>
</template>
