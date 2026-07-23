<!-- should generate diagnostics -->

<template>
  <!-- Missing value -->
  <div v-html></div>
  
  <!-- Empty value -->
  <div v-html=""></div>
  
  <!-- Unexpected argument -->
  <div v-html:aaa="foo"></div>
  
  <!-- Unexpected modifier -->
  <div v-html.bbb="foo"></div>
  
  <!-- Both argument and modifier -->
  <div v-html:aaa.bbb="foo"></div>
  
  <!-- Multiple modifiers -->
  <div v-html.bbb.ccc="foo"></div>
  
  <!-- With other directives (should still flag v-html) -->
  <div v-if="condition" v-html:arg="content" v-bind:class="className"></div>
</template>