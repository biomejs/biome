<!-- should not generate diagnostics -->
<script setup>
import { onMounted } from "vue";
onMounted(() => { document.title; });
</script>
<template><button @click="window.alert('hello')">Open</button></template>
