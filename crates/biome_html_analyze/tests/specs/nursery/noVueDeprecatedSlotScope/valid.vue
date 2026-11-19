<!-- should not generate diagnostics -->
<template>
  <LinkList>
    <template v-slot:name><a /></template>
  </LinkList>
  <LinkList>
    <template #name><a /></template>
  </LinkList>
  <LinkList>
    <template v-slot="{a}"><a /></template>
  </LinkList>
  <LinkList v-slot="{a}">
    <a />
  </LinkList>
  <LinkList>
    <template #default="{a}"><a /></template>
  </LinkList>
  <LinkList>
    <a slot="name" />
  </LinkList>
  <LinkList>
    <template><a /></template>
  </LinkList>
  <LinkList>
    <div data-slot-scope="props" />
  </LinkList>
</template>
