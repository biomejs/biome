<!-- should not generate diagnostics -->
<template>
  <my-component><div /></my-component>
  <div />
  <my-component :inline-template="foo"><div /></my-component>
  <my-component v-bind:inline-template="foo"><div /></my-component>
  <my-component Inline-Template="foo"><div /></my-component>
  <my-component inline-templates><div /></my-component>
  <my-component data-inline-template><div /></my-component>
</template>
