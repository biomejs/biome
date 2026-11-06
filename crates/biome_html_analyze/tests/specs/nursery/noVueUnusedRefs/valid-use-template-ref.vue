<!-- should not generate diagnostics -->
<script setup>
import { useTemplateRef } from "vue";

const first = useTemplateRef("x");
const second = useTemplateRef(`y`);
</script>

<template>
  <input ref="x" />
  <input ref="y" />
</template>
