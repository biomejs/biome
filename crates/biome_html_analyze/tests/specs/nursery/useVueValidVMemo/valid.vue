<!-- should not generate diagnostics -->
<template>
  <div v-memo="[x]"></div>
  <div v-memo="x"></div>
  <div v-memo="x ? y : z"></div>
  <div v-memo="x ? [y] : z"></div>
  <div v-memo="getDependencies()"></div>
  <div v-memo="x || []"></div>
  <div v-memo="a = [b]"></div>
  <div v-memo="(a, [b])"></div>
  <div v-memo="tag`template`"></div>
  <div v-memo="new Array(x)"></div>
  <div v-memo="x.dependencies"></div>
  <div v-for="i in items" v-memo="[i]"></div>
  <div v-for="i in items" :key="i.id" v-memo="[i.id === selected]">
    <span>{{ i.name }}</span>
  </div>
  <div v-for="i in items">
    <div v-for="j in i">{{ j }}</div>
  </div>
</template>
