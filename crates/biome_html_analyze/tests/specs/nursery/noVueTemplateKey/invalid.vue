<!-- should generate diagnostics -->
<template>
  <div><template key="foo"></template></div>
  <div><template v-bind:key="foo"></template></div>
  <div><template :key="foo"></template></div>
  <template v-slot="item" :key="item.id"><div /></template>
  <template #default="item" key="foo"><div /></template>
  <template v-if="ok" :key="foo"><div /></template>
  <template v-for="item in list">
    <template :key="item.id"><div /></template>
  </template>
</template>
