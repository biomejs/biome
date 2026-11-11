<!-- should generate diagnostics -->

<template>
  <!-- Missing value -->
  <div v-show></div>

  <!-- Empty value -->
  <div v-show=""></div>

  <!-- Blank value -->
  <div v-show="   "></div>

  <!-- Unexpected argument -->
  <div v-show:aaa="foo"></div>

  <!-- Unexpected dynamic argument -->
  <div v-show:[name]="foo"></div>

  <!-- Unexpected modifier -->
  <div v-show.aaa="foo"></div>

  <!-- Multiple modifiers -->
  <div v-show.aaa.bbb="foo"></div>

  <!-- Argument and modifier -->
  <div v-show:aaa.bbb="foo"></div>

  <!-- On a template element -->
  <template v-show="condition"><div></div></template>

  <!-- On a self-closing template element -->
  <template v-show="condition" />

  <!-- On a template element without a value -->
  <template v-show></template>

  <!-- On a component -->
  <MyComponent v-show:aaa="foo" />
</template>
