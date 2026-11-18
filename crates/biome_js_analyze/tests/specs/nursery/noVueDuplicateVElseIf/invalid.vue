<!-- should generate diagnostics -->
<template>
  <div v-if="foo" />
  <div v-else-if="foo" />

  <div v-if="isSomething(x)" />
  <div v-else-if="isSomething(x)" />

  <div v-if="a" />
  <div v-else-if="b" />
  <div v-else-if="c && d" />
  <div v-else-if="c && d" />

  <div v-if="n === 1" />
  <div v-else-if="n === 2" />
  <div v-else-if="n === 3" />
  <div v-else-if="n === 2" />
  <div v-else-if="n === 5" />

  <div v-if="a || b" />
  <div v-else-if="a" />

  <div v-if="a" />
  <div v-else-if="b" />
  <div v-else-if="a || b" />

  <div v-if="a" />
  <div v-else-if="a && b" />

  <div v-if="a && b" />
  <div v-else-if="a && b && c" />

  <div v-if="a || b" />
  <div v-else-if="b && c" />

  <div v-if="a" />
  <div v-else-if="b && c" />
  <div v-else-if="d && (c && e && b || a)" />

  <div v-if="foo" />
  <div v-else-if="foo && bar" />
  <div v-else-if="baz && foo" />

  <div v-if="a && b" />
  <div v-else-if="a && b && c" />
  <div v-else-if="a && c && b" />

  <div v-if="a || b" />
  <div v-else-if="a" />
  <div v-else-if="b" />

  <div v-if      ="((f && e) || d) && c || (b && a)" />
  <div v-else-if ="(a && b) || (c && (d || (e && f)))" />
  <div v-else-if ="(a && b) || (c && (d || (e && f)))" />
</template>
