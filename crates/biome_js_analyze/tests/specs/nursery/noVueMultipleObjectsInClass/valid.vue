<!-- should not generate diagnostics -->
<template>
  <div :class="[{ foo: isFoo }]" />
  <div :class="[{ foo: isFoo, bar: isBar }]" />
  <div :class="['baz', { foo: isFoo }, cls]" />
  <div :class="{ foo: isFoo, bar: isBar }" />
  <div :class="[[{ foo: isFoo }], { bar: isBar }]" />
  <div :class="[...extra, { bar: isBar }]" />
  <div :class="isFoo ? [{ foo: isFoo }, { bar: isBar }] : []" />
  <div v-foo:class="[{ foo: isFoo }, { bar: isBar }]" />
  <div :style="[{ color: 'red' }, { margin: 0 }]" />
  <div :[name]="[{ foo: isFoo }, { bar: isBar }]" />
  <div class="foo bar" />
  {{ [{ foo: isFoo }, { bar: isBar }] }}
</template>
