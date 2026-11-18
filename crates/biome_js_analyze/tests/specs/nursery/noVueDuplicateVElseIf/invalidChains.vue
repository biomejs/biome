<!-- should generate diagnostics -->
<template>
  <div v-if="a" /><div v-else-if="a" />
  <div v-if="a" /><div v-else-if="b" /><div v-else-if="a" /><div v-else-if="c" />
  <div v-if="a" /><div v-else-if="b" /><div v-else-if="c" /><div v-else-if="d" /><div v-else-if="b" /><div v-else-if="e" />
  <div v-if="a" /><div v-else-if="a" /><div v-else-if="a" />
  <div v-if="a" /><div v-else-if="b" /><div v-else-if="a" /><div v-else-if="b" /><div v-else-if="a" />
  <div v-if="a"><div v-if="b" /></div><div v-else-if="a" />
  <div v-if="a === 1" /><div v-else-if="a === 1" />
  <div v-if="1 < a" /><div v-else-if="1 < a" />
  <div v-if="true" /><div v-else-if="true" />
  <div v-if="a && b || c" /><div v-else-if="a && b || c" />
  <div v-if="f(a)" /><div v-else-if="f(a)" />
  <div v-if="a === 1" /><div v-else-if="a===1" />
  <div v-if="a === 1" /><div v-else-if="a === /* comment */ 1" />
  <div v-if="a || b" /><div v-else-if="b || a" />
  <div v-if="a || b" /><div v-else-if="c || d" /><div v-else-if="a || d" />
  <div v-if="(a === b && fn(c)) || d" /><div v-else-if="fn(c) && a === b" />
  <div v-if="a || c" /><div v-else-if="a && b || c" />
  <div v-if="a" /><div v-else-if="b" /><div v-else-if="c && a || b" />
  <div v-if="a" /><div v-else-if="b" /><div v-else-if="c && (a || b)" />
  <div v-if="a" /><div v-else-if="b && c" /><div v-else-if="d && (a || e && c && b)" />
  <div v-if="a || b && c" /><div v-else-if="b && c && d" />
  <div v-if="a" /><div v-else-if="b" /><div v-else-if="(a || b) && c" />
  <div v-if="(a && (b || c)) || d" /><div v-else-if="(c || b) && e && a" />
  <div v-if="a && b || b && c" /><div v-else-if="a && b && c" />
  <div v-if="a || (b && (c || d))" /><div v-else-if="(d || c) && b" />
  <div v-if="a || b" /><div v-else-if="(b || a) && c" />
  <div v-if="a || b" /><div v-else-if="c" /><div v-else-if="d" /><div v-else-if="b && (a || c)" />
  <div v-if="a || b || c" /><div v-else-if="a || (b && d) || (c && e)" />
  <div v-if="a || (b || c)" /><div v-else-if="a || (b && c)" />
  <div v-if="a || b" /><div v-else-if="c" /><div v-else-if="d" /><div v-else-if="(a || c) && (b || d)" />
  <div v-if="a" /><div v-else-if="b" /><div v-else-if="c && (a || d && b)" />
  <div v-if="a" /><div v-else-if="a || a" />
  <div v-if="a || a" /><div v-else-if="a || a" />
  <div v-if="a || a" /><div v-else-if="a" />
  <div v-if="a" /><div v-else-if="a && a" />
  <div v-if="a && a" /><div v-else-if="a && a" />
  <div v-if="a && a" /><div v-else-if="a" />
</template>
