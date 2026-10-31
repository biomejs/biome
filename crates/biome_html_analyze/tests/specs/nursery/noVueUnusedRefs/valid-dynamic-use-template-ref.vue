<!-- should not generate diagnostics -->
<script setup>
import { useTemplateRef } from "vue";

const props = defineProps(["name"]);
const element = useTemplateRef(props.name);
</script>

<template>
  <input ref="x" />
</template>
