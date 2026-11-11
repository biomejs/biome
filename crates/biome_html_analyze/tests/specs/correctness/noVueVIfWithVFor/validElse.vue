<!-- should not generate diagnostics -->
<template>
  <div v-if="foo">{{ x }}</div>
  <div v-else-if="foo">{{ x }}</div>
  <div v-else>{{ x }}</div>

  <div v-for="x in xs">{{ x }}</div>

  <div v-if="foo">foo</div>
  <template v-else-if="bar">
    <div v-for="x in xs">{{ x }}</div>
  </template>
  <template v-else>
    <div v-for="x in xs">{{ x }}</div>
  </template>
</template>
