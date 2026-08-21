<!-- should not generate diagnostics -->
<script setup>
const isHighlighted = ref(false);
const vHighlight = {
  mounted(el) {
    el.style.color = 'red';
  }
};
</script>

<template>
  <!-- custom directive -->
  <div v-highlight>Highlighted</div>

  <!-- custom directive with value -->
  <div v-highlight="isHighlighted">Highlighted</div>
</template>
