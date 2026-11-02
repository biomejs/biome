<!-- should not generate diagnostics -->
<template>
  <LinkList>
    <one slot="one" />
    <MyComponent slot="MyComponent-slot" />
    <a :slot="name" />
  </LinkList>
  <link-list>
    <one slot="one" />
  </link-list>
  <my-web-component>
    <span slot="header">Title</span>
  </my-web-component>
</template>
