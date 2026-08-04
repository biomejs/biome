<!-- should not generate diagnostics -->
<script>
import Child from "./invalid.vue";
</script>

<style>
:root { --global-color: green; }
</style>
<style scoped>
:root { --local-color: red; }
:global(:root) { --selector-global-color: blue; }
</style>

<template><Child /></template>
