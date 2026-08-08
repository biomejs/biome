<!-- should not generate diagnostics -->
<template>
  <RouterView v-slot="{ Component }">
    <component :is="Component" />
  </RouterView>

  <template v-slot:foo="{ x }">
    <span>{{ x }}</span>
  </template>

  <RouterView #default="{ y }">
    <span>{{ y }}</span>
  </RouterView>

  <RouterView v-slot="slotProps">
    <span>{{ slotProps }}</span>
  </RouterView>

  <RouterView v-slot="{ item = {} }">
    <span>{{ item }}</span>
  </RouterView>

  <RouterView v-slot="[first, ...others]">
    <span>{{ first }}{{ others }}</span>
  </RouterView>

  <!--
    Known limitation: slot bindings are collected for the whole document rather
    than scoped to the element that declares them, so `item` still resolves
    here even though Vue does not provide it outside the RouterView above.
    This matches how `v-for` bindings are collected today.
  -->
  <span>{{ item }}</span>
</template>
