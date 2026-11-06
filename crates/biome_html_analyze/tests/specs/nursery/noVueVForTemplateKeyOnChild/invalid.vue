<!-- should generate diagnostics -->
<template>
  <div>
    <template v-for="x in list"><Foo :key="x" /></template>
    <template v-for="x in list"><Foo :key="x.id" /></template>
    <template v-for="x in list" :key="foo"><Foo :key="x.id" /></template>
    <template v-for="x in list" :key><div /><Foo :key="x.id" /></template>
    <template v-for="x in list" :key><Foo :key="'foo' + x.id" /><Bar :key="'bar' + x.id" /></template>
    <template v-for="x in list"><div v-bind:key="x.id"></div></template>
    <template v-for="(x, i) in list"><Foo :key="i" /></template>
    <template v-for="({ id }, i) in list"><Foo :key="id" /></template>
    <template v-for="{ id: itemId } in list"><Foo :key="itemId" /></template>
    <template v-for="x in list" :key="ids.find((x) => x.ok)"><Foo :key="x.id" /></template>
    <template v-for="x in list">
      <Foo :key="x.error1" />
      <div :key="x.error2" />
      <slot :key="x.error3"></slot>
    </template>
  </div>
</template>
