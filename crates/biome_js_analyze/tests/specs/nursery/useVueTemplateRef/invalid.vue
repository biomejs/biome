<!-- should generate diagnostics -->
<template>
  <div ref="root" />
  <h1 ref="heading">Heading</h1>
  <a href="" ref="link">Link</a>
  <button ref="button">Content</button>
</template>

<script setup>
import { ref, shallowRef } from "vue";

const root = ref();
const heading = ref(null);
const link = shallowRef();
const buttonRef = ref();
</script>
