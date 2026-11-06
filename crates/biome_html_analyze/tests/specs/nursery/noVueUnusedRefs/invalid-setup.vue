<!-- should generate diagnostics -->
<script setup lang="ts">
import { useTemplateRef } from "vue";

const props = defineProps(["label"]);
type box = HTMLDivElement;
const input = useTemplateRef("input");
</script>

<template>
  <input ref="input" />
  <span ref="label" />
  <div ref="box" />
</template>
