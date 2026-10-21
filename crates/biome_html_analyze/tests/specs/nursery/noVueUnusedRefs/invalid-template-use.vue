<!-- should generate diagnostics -->
<template>
  <button @click="alert($refs.foo.value)">Click</button>
  <input ref="foo" />
  <input ref="x" />
</template>
