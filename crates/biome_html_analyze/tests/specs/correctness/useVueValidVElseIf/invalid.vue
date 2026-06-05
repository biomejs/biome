<!-- should generate diagnostics -->

<template>
  <!-- Argument: v-else-if with argument -->
  <div v-if="a"></div><div v-else-if:arg="b"></div>

  <!-- Modifier: v-else-if with modifier -->
  <div v-if="a"></div><div v-else-if.mod="b"></div>

  <!-- Missing value: v-else-if without value -->
  <div v-if="a"></div><div v-else-if></div>

  <!-- Dangling: v-else-if without previous conditional -->
  <div v-else-if="b"></div>

  <!-- Conflicts with v-if on same element -->
  <div v-if="a" v-else-if="b"></div>

  <!-- Conflicts with v-else on same element -->
  <div v-else v-else-if="b"></div>

  <!-- Preceded by unrelated element -->
  <span></span><div v-else-if="b"></div>

  <!-- Preceded by comment only -->
  <!-- comment --><div v-else-if="b"></div>

  <!-- Multiple modifiers -->
  <div v-if="a"></div><div v-else-if.mod1.mod2="b"></div>

  <!-- Complex argument case -->
  <div v-if="a"></div><div v-else-if:[complex]="b"></div>
</template>