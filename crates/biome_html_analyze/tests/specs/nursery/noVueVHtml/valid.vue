<!-- should not generate diagnostics -->
<template>
  <div>{{ content }}</div>
  <div v-text="content"></div>
  <div :html="content"></div>
  <div v-bind:html="content"></div>
  <div :innerHTML="content"></div>
  <div html="content"></div>
  <div v-html-custom="content"></div>
</template>
