<!-- should generate diagnostics -->
<script setup>
function useTemplateRef(name) {
  return name;
}
useTemplateRef("input");
</script>

<template>
  <input ref="input" />
</template>
