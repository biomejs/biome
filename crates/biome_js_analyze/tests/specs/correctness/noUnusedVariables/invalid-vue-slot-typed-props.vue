<!-- should generate diagnostics -->
<script setup lang="ts">
import Component from "./Component.vue";

interface ValueType {
  innerValue: string;
}
interface UnusedType {
  innerValue: string;
}
</script>

<template>
  <Component v-slot="{ value }: { value: ValueType }">{{ value.innerValue }}</Component>
</template>
