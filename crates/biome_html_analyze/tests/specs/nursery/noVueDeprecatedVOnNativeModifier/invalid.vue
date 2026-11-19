<!-- should generate diagnostics -->
<template>
  <MyComponent v-on:click.native="onClick" />
  <MyComponent @click.native="onClick" />
  <input v-on:keyup.native="fire">
  <input v-on:keyup.foo.native.bar="fire">
  <input @keyup.enter.native="fire">
  <MyComponent v-on:[event].native="onEvent" />
  <MyComponent @[event].native="onEvent" />
  <MyComponent @click.native />
  <MyComponent v-on.native="handlers" />
</template>
