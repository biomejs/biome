<!-- should not generate diagnostics -->
<template>
  <div v-text="content"></div>
  <div v-html="html"></div>
  <span v-text="content" />
  <MyComponent v-if="content" />
  <MyComponent>{{ content }}</MyComponent>
  <MyComponent><span v-html="html"></span></MyComponent>
  <svg><g v-text="content" /></svg>
  <svg><text v-text="content" /></svg>
  <g v-text="content" />
  <math><mi v-text="content" /></math>
  <math><mspace v-text="content" /></math>
  <mi v-text="content" />
  <annotation-xml v-html="html"></annotation-xml>
  <template v-if="visible"><p v-text="content"></p></template>
  <div :is="component" v-text="content"></div>
  <div is="my-button" v-text="content"></div>
  <textarea v-text="content"></textarea>
  <MyComponent :text="content" />
</template>
