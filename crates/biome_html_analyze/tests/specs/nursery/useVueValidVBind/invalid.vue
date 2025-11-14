<!-- should generate diagnostics -->

<template>
  <!-- Missing argument: long-form without an argument -->
  <div v-bind></div>
  <div v-bind />
  <Foo v-bind />

  <!-- Missing value -->
  <Foo v-bind:foo />
  <Foo :foo />

  <!-- Missing argument with modifier -->
  <div v-bind.prop></div>

  <!-- Invalid single modifier on long-form -->
  <div v-bind:foo.invalid="bar"></div>

  <!-- Invalid modifier on shorthand -->
  <span :bar.badModifier="baz"></span>

  <!-- Mixed valid and invalid modifiers: 'prop' is valid, 'wrong' is not -->
  <p :baz.prop.wrong="value"></p>

  <!-- Dynamic argument is present but modifier is invalid -->
  <p v-bind:[dynamic].notAValidModifier="value"></p>

  <!-- Multiple invalid modifiers -->
  <button :disabled.once="true"></button>

  <!-- Component binding with unknown modifier -->
  <MyComponent v-bind:propName.weird="someValue"></MyComponent>
</template>
