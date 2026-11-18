<!-- should not generate diagnostics -->
<template>
  <div ref="root" />
</template>

<script setup>
function ref() {
  return null;
}

const root = ref();
</script>
