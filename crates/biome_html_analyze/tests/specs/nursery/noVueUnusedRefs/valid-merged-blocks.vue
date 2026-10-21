<!-- should not generate diagnostics -->
<script>
export default {
  mounted() {
    this.$refs.a.focus();
  },
};
</script>

<script setup>
import { useTemplateRef } from "vue";

const b = useTemplateRef("b");
</script>

<template>
  <input ref="a" />
  <input ref="b" />
</template>
