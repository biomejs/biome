<!-- should not generate diagnostics -->
<template>
  <LinkList>
    <one slot="one" />
    <two slot="two" />
    <my-component slot="my-component-slot" />
    <myComponent slot="myComponent-slot" />
    <MyComponent slot="MyComponent-slot" />
    <MyComponent :slot="name" />
  </LinkList>
</template>
