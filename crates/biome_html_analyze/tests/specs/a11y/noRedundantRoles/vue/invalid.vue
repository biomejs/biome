<!-- should generate diagnostics -->
<template>
  <article role="article"></article>
  <button role="button"></button>
  <h1 role="heading" aria-level="1">title</h1>
  <nav role="navigation"></nav>
  <!-- static Vue bindings with redundant roles should also be flagged -->
  <article :role="'article'"></article>
  <button v-bind:role="'button'"></button>
  <nav :role="'navigation'"></nav>
</template>
