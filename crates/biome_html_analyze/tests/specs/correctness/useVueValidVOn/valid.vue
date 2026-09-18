<!-- should not generate diagnostics -->

<template>
  <!-- Basic long-form and shorthand event handlers -->
  <div v-on:click="foo"></div>
  <div @click="foo"></div>

  <!-- Valid modifiers from the rule -->
  <div v-on:click.stop="foo"></div>
  <div v-on:click.prevent="foo"></div>
  <div v-on:click.capture="foo"></div>
  <div v-on:click.self="foo"></div>
  <div v-on:click.once="foo"></div>
  <div v-on:click.passive="foo"></div>
  <div v-on:click.exact="foo"></div>

  <!-- System modifiers -->
  <div @keydown.ctrl="handler"></div>
  <div @keydown.shift="handler"></div>
  <div @keydown.alt="handler"></div>
  <div @keydown.meta="handler"></div>

  <!-- Mouse modifiers -->
  <div @mousedown.left="handler"></div>
  <div @mousedown.right="handler"></div>
  <div @mousedown.middle="handler"></div>

  <!-- Keyboard key aliases -->
  <div @keydown.enter="handler"></div>
  <div @keydown.tab="handler"></div>
  <div @keydown.delete="handler"></div>
  <div @keydown.esc="handler"></div>
  <div @keydown.space="handler"></div>
  <div @keydown.up="handler"></div>
  <div @keydown.down="handler"></div>
  <div @keydown.left="handler"></div>
  <div @keydown.right="handler"></div>
  <div @keydown.arrow-down="handler"></div>
  <div @keydown.a="handler"></div>
  <div @keydown.b="handler"></div>
  <div @keydown.a.b.c="handler"></div>
  <div @keydown.27="foo"></div>

  <!-- Combined modifiers -->
  <div @click.stop.prevent="foo"></div>
  <div @keyup.enter.exact="onKey"></div>

  <!-- Dynamic argument -->
  <div v-on:[event]="handler"></div>
  <div @[event]="handler"></div>

  <!-- Multiple v-on directives on same element -->
  <div v-on:click="foo" @mouseenter="bar"></div>

  <!-- Object syntax: arg-less v-on with a value spreads the object's
       listeners onto the element. -->
  <div v-on="$listeners"></div>
  <div v-on="listeners"></div>

  <!-- Component event handlers -->
  <MyComponent @click="handler" @custom-event="customHandler"></MyComponent>

  <!-- Native modifier (Vue 2 compatibility) -->
  <div @click.native="handler"></div>

  <!-- Verb modifiers (`stop`, `prevent`) carry an intrinsic side
       effect and are valid without a handler. -->
  <div @click.stop></div>
  <form @submit.prevent></form>
  <div v-on:click.stop></div>
  <div v-on:submit.prevent></div>
  <div @click.stop.prevent></div>
  <div @click.prevent.exact></div>

  <!-- Verb-modifier-only on a self-closing component (modifier is
       followed by whitespace + `/>`, so the modifier-name lookup must
       trim trivia). -->
  <Foo @click.stop />
  <Foo @submit.prevent />
  <Foo v-on:click.stop />
</template>
