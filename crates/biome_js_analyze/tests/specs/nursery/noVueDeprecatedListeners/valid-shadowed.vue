<!-- should not generate diagnostics -->
<script setup>
const $listeners = { click: () => null };
</script>

<template>
  <div v-on="$listeners" />
</template>
