<!-- should generate diagnostics -->
<template>
  <div v-if="foo." />
  <div v-else-if="foo." />

  <div v-else-if="foo." />
  <div v-else-if="foo" />

  <div v-if="foo." />
  <div v-else-if="bar" />
  <div v-else-if="bar" />
</template>
