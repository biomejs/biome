<!-- should not generate diagnostics -->
<script setup>
let counter = 0;
</script>
<template>
  <!-- shorthand @click -->
  <button type="button" @click="counter += 1">+</button>
  <!-- longhand v-on:click -->
  <button type="button" v-on:click="counter -= 1">-</button>
</template>
