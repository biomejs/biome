<!-- should generate diagnostics -->
<template>
  <my-component inline-template><div /></my-component>
  <my-component inline-template=""><div /></my-component>
  <my-component inline-template="foo"><div /></my-component>
  <MyComponent inline-template>
    <div>{{ message }}</div>
  </MyComponent>
  <div>
    <my-component class="a" inline-template v-if="ok"><span /></my-component>
  </div>
</template>
