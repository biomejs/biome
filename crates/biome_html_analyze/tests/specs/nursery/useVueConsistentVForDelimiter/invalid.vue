<!-- should generate diagnostics -->
<template>
  <div v-for="x of xs"></div>
  <div v-for="(x, index) of xs"></div>
  <div v-for="x    of xs"></div>
  <div v-for="x of    xs"></div>
  <div v-for="x    of    xs"></div>
  <div v-for="{ x, y } of xs"></div>
  <div v-for='x of xs'></div>
  <MyItem v-for="item of items" :key="item.id" />
  <template v-for="n of 10">{{ n }}</template>
</template>
