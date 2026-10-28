<!-- should generate diagnostics -->
<template>
  <div v-memo="{x}" />
  <div v-memo="a ? {b} : c+d" />
  <div v-memo="(a,{b},c(),d+1)" />
  <div v-memo="()=>42" />
  <div v-memo="function () { return [] }" />
  <div v-memo="a=42" />
  <div v-memo="'string'" />
  <div v-memo="42" />
  <div v-memo="null" />
  <div v-memo="`template ${x}`" />
  <div v-memo="!x" />
  <div v-memo="typeof x" />
  <div v-memo="x++" />
  <div v-memo="--x" />
  <div v-memo="'a' in x" />
  <div v-memo="x instanceof Array" />
  <div v-memo="class {}" />
  <div v-memo="({ x })" />
</template>
