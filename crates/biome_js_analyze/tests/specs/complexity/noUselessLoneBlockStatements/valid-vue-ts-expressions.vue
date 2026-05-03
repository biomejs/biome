<!-- should not generate diagnostics -->
<script setup lang="ts">
const num = 1
</script>

<template>
  <h1>{{ num as any }}</h1>
</template>
