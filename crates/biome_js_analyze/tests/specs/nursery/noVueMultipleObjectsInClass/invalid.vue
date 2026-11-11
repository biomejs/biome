<!-- should generate diagnostics -->
<template>
  <div :class="[{ foo: isFoo }, { bar: isBar }]" />
  <div v-bind:class="[{ foo: isFoo }, { bar: isBar }]" />
  <div :class="['baz', { foo: isFoo }, cls, { bar: isBar }]" />
  <div :class="[, { foo: isFoo }, { bar: isBar }]" />
  <div :class="[{ foo: isFoo }, ({ bar: isBar })]" />
  <div :class="([{ foo: isFoo }, { bar: isBar }])" />
  <div :class="[{ foo: isFoo }, { bar: isBar }, { qux: isQux }]" />
  <div :class.camel="[{}, {}]" />
</template>
