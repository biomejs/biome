<!-- should not generate diagnostics -->
<template>
  <LinkList>
    <template v-slot:name><a /></template>
  </LinkList>
  <LinkList>
    <template #name><a /></template>
  </LinkList>
  <LinkList>
    <template #[slotName]><a /></template>
  </LinkList>
  <LinkList>
    <template v-slot="{ item }"><a /></template>
  </LinkList>
  <LinkList v-slot="{ item }">
    <a />
  </LinkList>
  <LinkList>
    <template #default="{ item }"><a /></template>
  </LinkList>
  <LinkList>
    <template><a /></template>
  </LinkList>
  <LinkList>
    <a />
  </LinkList>
  <!-- Attributes whose names only contain "slot" -->
  <LinkList>
    <a :slots="slots" data-slot="name" slot-name="name" :slotName="name" />
  </LinkList>
  <!-- A <slot> element is the way to declare a slot inside a component -->
  <slot name="header" />
</template>
