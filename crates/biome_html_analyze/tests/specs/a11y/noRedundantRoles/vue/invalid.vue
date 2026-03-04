<template>
  <!-- Native elements with redundant roles should still trigger -->
  <article role="article"></article>
  <button role="button"></button>
  <h1 role="heading" aria-level="1">title</h1>
  <nav role="navigation"></nav>
</template>
