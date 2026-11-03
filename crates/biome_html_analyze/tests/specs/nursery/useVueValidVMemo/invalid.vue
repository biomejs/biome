<!-- should generate diagnostics -->
<template>
  <div v-memo:aaa="[x]"></div>
  <div v-memo.aaa="[x]"></div>
  <div v-memo.aaa.bbb="[x]"></div>
  <div v-memo></div>
  <div v-memo=""></div>
  <div v-memo="  "></div>
  <div v-for="i in items">
    <div v-memo="[x]" />
  </div>
  <template v-for="i in items">
    <span>
      <div v-memo="[i]"></div>
    </span>
  </template>
  <div v-for="i in items">
    <div v-for="j in i" v-memo="[j]"></div>
  </div>
</template>
