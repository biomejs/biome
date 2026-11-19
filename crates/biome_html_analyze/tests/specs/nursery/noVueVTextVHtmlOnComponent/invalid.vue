<!-- should generate diagnostics -->
<template>
  <MyComponent v-text="content" />
  <MyComponent v-html="html" />
  <MyComponent v-text="content"></MyComponent>
  <my-component v-html="html"></my-component>
  <router-link v-text="content" />
  <UI.Button v-text="content" />
  <component :is="component" v-text="content" />
  <component :is="component" v-html="html" />
  <component v-text="content" />
  <unknownelement v-text="content"></unknownelement>
  <Transition v-html="html"></Transition>
  <keep-alive v-text="content"></keep-alive>
  <tr is="vue:my-row" v-text="content"></tr>
  <div>
    <MyComponent v-if="visible" v-html="html" />
  </div>
  <svg><MyIcon v-text="content" /></svg>
</template>
