<!-- should not generate diagnostics -->
<template>
  <div v-if="foo" />
  <div v-else-if="bar" />
  <div v-else-if="baz" />

  <div v-if="foo">
    <div v-else-if="foo" />
  </div>

  <div v-if="foo" />
  <div v-else-if="bar" />
  <div v-if="bar" />
  <div v-else-if="foo" />

  <div v-if="isSomething(x)" />
  <div v-else-if="isSomethingElse(x)" />

  <div v-if="a" />
  <div v-else-if="b" />
  <div v-else-if="c && d" />
  <div v-else-if="c && e" />

  <div v-if="n === 1" />
  <div v-else-if="n === 2" />
  <div v-else-if="n === 3" />
  <div v-else-if="n === 4" />
  <div v-else-if="n === 5" />

  <div v-if="foo" />
  <div />
  <div v-else-if="foo" />

  <div v-if="foo" />
  <div v-else />
  <div v-else-if="foo" />

  <div v-if />
  <div v-else-if />

  <div v-if="a" /><div v-else-if="b" />
  <div v-if="a" /><div v-else-if="b" /><div v-else-if="c" />
  <div v-if="true" /><div v-else-if="false" />
  <div v-if="1" /><div v-else-if="2" />
  <div v-if="f" /><div v-else-if="f()" />
  <div v-if="f(a)" /><div v-else-if="g(a)" />
  <div v-if="f(a)" /><div v-else-if="f(b)" />
  <div v-if="a === 1" /><div v-else-if="a === 2" />
  <div v-if="a === 1" /><div v-else-if="b === 1" />
  <div v-if="a" />
  <div v-if="a"><div v-if="a" /></div>
  <div v-if="a"><div v-if="b" /></div><div v-else-if="b" />
  <div v-if="a"><div v-if="b" /><div v-else-if="a" /></div>
  <div v-if="a" /><div v-else-if="!!a" />
  <div v-if="a === 1" /><div v-else-if="a === (1)" />
  <div v-if="a || b" /><div v-else-if="c || d" />
  <div v-if="a || b" /><div v-else-if="a || c" />
  <div v-if="a" /><div v-else-if="a || b" />
  <div v-if="a" /><div v-else-if="b" /><div v-else-if="a || b || c" />
  <div v-if="a && b" /><div v-else-if="a" /><div v-else-if="b" />
  <div v-if="a && b" /><div v-else-if="b && c" /><div v-else-if="a && c" />
  <div v-if="a && b" /><div v-else-if="b || c" />
  <div v-if="a" /><div v-else-if="b && (a || c)" />
  <div v-if="a" /><div v-else-if="b && (c || d && a)" />
  <div v-if="a && b && c" /><div v-else-if="a && b && (c || d)" />
  <div v-if="a ?? b" /><div v-else-if="b ?? a" />
  <div v-if="!(a && b)" /><div v-else-if="a" />
</template>
