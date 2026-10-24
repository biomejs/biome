<!-- should not generate diagnostics -->
<template>
  <button @click="emit('submit')">Submit</button>
</template>

<script setup lang="ts">
import type { Emits } from "./types";

const emit = defineEmits<Emits>();

emit("change");
</script>
