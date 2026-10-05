<!-- should not generate diagnostics -->
<script setup>
const $scopedSlots = { default: () => null };
</script>

<template>
  {{ $scopedSlots.default }}
</template>
