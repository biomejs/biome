<!-- should generate diagnostics -->
<template>
  <LinkList>
    <template slot><a /></template>
  </LinkList>
  <LinkList>
    <template slot="name"><a /></template>
  </LinkList>
  <LinkList>
    <template slot="name" slot-scope="{ item }"><a /></template>
  </LinkList>
  <LinkList>
    <template slot="f o o"><a /></template>
  </LinkList>
  <LinkList>
    <template :slot="slotName"><a /></template>
  </LinkList>
  <LinkList>
    <template v-bind:slot="slotName"><a /></template>
  </LinkList>
  <LinkList>
    <template :slot><a /></template>
  </LinkList>
  <LinkList>
    <a slot="name" />
  </LinkList>
  <LinkList>
    <a :slot="name" />
  </LinkList>
  <my-component>
    <div slot="header">Header</div>
  </my-component>
  <component :is="tag">
    <div slot="named">Content</div>
  </component>
  <some-component>
    <template v-if="condition">
      <template slot="some-slot">Nested</template>
    </template>
  </some-component>
</template>
