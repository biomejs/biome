<!-- should generate diagnostics -->

<template>
  <!-- Missing event name: long-form without an argument -->
  <div v-on></div>

  <!-- Missing event name with value -->
  <div v-on="foo"></div>

  <!-- Missing handler on long-form -->
  <div v-on:click></div>

  <!-- Missing handler on shorthand -->
  <div @click></div>

  <!-- Invalid single modifier on long-form -->
  <div v-on:click.bogus="foo"></div>

  <!-- Invalid modifier on shorthand -->
  <span @click.badModifier="foo"></span>

  <!-- Mixed valid and invalid modifiers: 'stop' is valid, 'wrong' is not -->
  <p @click.stop.wrong="foo"></p>

  <!-- Dynamic argument is present but modifier is invalid -->
  <p v-on:[event].notAValidModifier="foo"></p>

  <!-- Multiple invalid modifiers -->
  <button @submit.invalidModifier.anotherBad="handler"></button>

  <!-- Component with invalid modifier -->
  <MyComponent @click.weird="someHandler"></MyComponent>

  <!-- Missing handler with valid modifiers -->
  <div @keyup.enter></div>
</template>