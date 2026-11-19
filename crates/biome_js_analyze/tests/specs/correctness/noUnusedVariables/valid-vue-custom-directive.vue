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
  <div v-highlight>Highlighted</div>

  <div v-highlight="isHighlighted">Highlighted</div>
</template>
