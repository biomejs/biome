<!-- should not generate diagnostics -->
<script setup>
import { ref } from "vue";
import imported from "./imported";

const input = ref(null);
let button;
function handler() {}
</script>

<template>
  <input ref="input" />
  <button ref="button" />
  <div ref="imported" />
  <div ref="handler" />
</template>
