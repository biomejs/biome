<!-- should not generate diagnostics -->

<template>
  <!-- Plain HTML attributes without uppercase letters are valid -->
  <div class="container" id="root" data-test-id="z" aria-label="label" some-attr="ok" some_attr="ok"></div>

  <!-- v-bind with static hyphenated argument -->
  <div v-bind:foo-bar="bar"></div>

  <!-- v-bind shorthand with static hyphenated argument -->
  <div :foo-bar="bar"></div>

  <!-- v-bind with dynamic argument (not checked by the rule) -->
  <div v-bind:[dynamicArg]="val"></div>

  <!-- v-on shorthand: event names are lowercase/hyphenated -->
  <button @click="onClick" @keydown.enter="onEnter"></button>

  <!-- Custom components using hyphenated props -->
  <MyComp some-prop="x" another-prop="y"></MyComp>
  <my-comp some-prop="x" :another-prop="y"></my-comp>

  <!-- Additional valid attributes -->
  <input type="text" aria-label="user-name" data-user-id="123" />
  <Panel pt:root:class="border border-solid" pt:header:id="headerId" pt:header:data-test-id="testId" />

  <!-- SVG attributes are not Vue props -->
  <svg version="1" viewBox="0 0 320 180" xmlns="http://www.w3.org/2000/svg" xmlns:cc="http://creativecommons.org/ns#">
    <rect strokeWidth="2"></rect>
    <linearGradient gradientUnits="userSpaceOnUse"></linearGradient>
  </svg>

  <!-- Should not apply to other directives -->
  <MyComp v-foo:bar="asdf"></MyComp>
</template>
