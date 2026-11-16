<!-- should generate diagnostics -->
<template>
  <p>{{ msg | filter }}</p>
  <p>{{ msg | filter(x) }}</p>
  <p>{{ msg | filterA | filterB }}</p>
  <div v-for="msg in messages">{{ msg | filter }}</div>
  <div v-bind:id="msg | filter"></div>
  <div v-bind:id="msg | filter(aaa)"></div>
  <div v-bind:id="msg | filterA | filterB"></div>
  <div :id="msg | filter"></div>
  <div :[key]="msg | filter"></div>
  <div v-bind="attrs | filter"></div>
  <p>{{ a || b | filter }}</p>
  <p>{{ ok ? a : b | filter }}</p>
  <Field :format="value => value | currency"></Field>
</template>
