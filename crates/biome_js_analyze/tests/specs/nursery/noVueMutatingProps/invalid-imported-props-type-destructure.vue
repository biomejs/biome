<!-- should generate diagnostics -->
<script setup lang="ts">
import type { Props } from "./types";

let { list, user } = defineProps<Props>();

function update() {
  list.push(1);
  user.name = "Alice";
}
</script>

<template>
  <button @click="list.pop()">Remove</button>
</template>
