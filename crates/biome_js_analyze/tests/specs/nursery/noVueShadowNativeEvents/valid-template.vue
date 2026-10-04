<!-- should not generate diagnostics -->
<template>
  <button @click="$emit('save')">Save</button>
  <button @click="$emit(eventName)">Dynamic</button>
  <button @click="emitter.$emit('click')">Other</button>
  <button @click="$fire('click')">Other</button>
</template>

<script setup>
defineEmits(["save"]);
</script>
