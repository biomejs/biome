<!-- should not generate diagnostics -->
<template>
  <div v-for="x in xs"></div>
  <div v-for="x    in xs"></div>
  <div v-for="x in    xs"></div>
  <div v-for="x    in    xs"></div>
  <div v-for="(value, key, index) in xs"></div>
  <div v-for="{ x, y } in xs"></div>
  <div v-for='x in xs'></div>
  <MyItem v-for="item in items" :key="item.id" />
  <template v-for="n in 10">{{ n }}</template>
  <div v-if="ok">of</div>
</template>
