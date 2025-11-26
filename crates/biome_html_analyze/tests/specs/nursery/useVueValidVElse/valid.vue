<!-- should not generate diagnostics -->

<template>
  <!-- Simple v-if/v-else chain -->
  <div v-if="condition"></div>
  <div v-else></div>

  <!-- v-if/v-else-if/v-else chain -->
  <div v-if="condition1"></div>
  <div v-else-if="condition2"></div>
  <div v-else></div>

  <!-- Whitespace and comments between elements -->
  <div v-if="condition"></div>
  <!-- comment -->
  <div v-else></div>

  <!-- Newline between elements -->
  <div v-if="condition"></div>
  <div v-else></div>

  <!-- Nested templates -->
  <template>
    <div v-if="condition"></div>
    <div v-else></div>
  </template>

  <!-- Multiple separate chains -->
  <div v-if="cond1"></div>
  <div v-else></div>
  <span v-if="cond2"></span>
  <span v-else></span>
</template>