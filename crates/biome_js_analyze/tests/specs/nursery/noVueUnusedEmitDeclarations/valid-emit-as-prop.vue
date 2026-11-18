<!-- should not generate diagnostics -->
<template>
  <ChildForm :notify="emit" />
</template>

<script setup>
import ChildForm from "./ChildForm.vue";

const emit = defineEmits(["foo"]);
</script>
