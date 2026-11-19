<!-- should generate diagnostics -->
<template>
  <div v-html="content"></div>
  <div v-html="'<p>' + text + '</p>'"></div>
  <div v-html></div>
  <div v-html:arg.mod="content"></div>
  <div v-HTML="content"></div>
  <MyComponent v-if="visible" v-html="content" />
</template>
