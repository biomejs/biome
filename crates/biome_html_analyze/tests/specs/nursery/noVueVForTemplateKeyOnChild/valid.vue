<!-- should not generate diagnostics -->
<template>
  <div>
    <template v-for="x in list"><Foo /></template>
    <template v-for="x in list" :key="x"><Foo /></template>
    <template v-for="x in list" :key="x.id"><Foo :key="x.id" /></template>
    <template v-for="(x, i) in list" :key="i"><Foo :key="x" /></template>
    <template v-for="(x, i) in list"><Foo :key="foo" /></template>
    <template v-for="x in list" v-bind:key="x.id"><Foo :key="x.id" /></template>
    <template v-for="{ id: itemId } in list"><Foo :key="id" /></template>
    <template v-for="x in list"><Foo key="x" /></template>
    <template v-for="x in list"><div><Foo :key="x" /></div></template>
    <template v-if="show"><Foo :key="x" /></template>
    <div v-for="x in list"><Foo :key="x" /></div>
    <template v-for="x in list"><Foo :key="ids.find((x) => x.ok)" /></template>
    <template v-for="x in list">
      <Foo v-if="a" :key="x.key1" />
      <Foo v-else-if="a" :key="x.key2" />
      <Foo v-else :key="x.key3" />
      <Foo v-for="y in list" :key="x.key4" />
    </template>
  </div>
</template>
