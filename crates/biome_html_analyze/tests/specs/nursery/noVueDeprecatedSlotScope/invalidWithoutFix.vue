<!-- should generate diagnostics -->
<template>
  <LinkList>
    <template slot-scope="{a}" slot="foo"><a /></template>
  </LinkList>
  <LinkList>
    <template slot-scope="{a}" :slot="arg"><a /></template>
  </LinkList>
  <LinkList>
    <template v-for="x in xs" slot-scope="{a}">{{ a }}</template>
  </LinkList>
  <LinkList>
    <template slot-scope="{a}">{{ a }}</template>
    <div />
  </LinkList>
  <LinkList>
    <template slot-scope="{a}">{{ a }}</template>
    Some text
  </LinkList>
  <LinkList>
    <template slot-scope="{a}">{{ a }}</template>
    <template #default>Default</template>
  </LinkList>
  <LinkList>
    <a v-if="ready" slot-scope="{a}" />
  </LinkList>
  <LinkList v-slot="outer">
    <template slot-scope="{a}">{{ a }}</template>
  </LinkList>
  <div>
    <template slot-scope="{a}">{{ a }}</template>
  </div>
  <component :is="list">
    <template slot-scope="{a}">{{ a }}</template>
  </component>
</template>
