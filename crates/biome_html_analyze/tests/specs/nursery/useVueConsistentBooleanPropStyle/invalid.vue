<!-- should generate diagnostics -->
<template>
  <MyComp :show="true" />
  <MyComp v-bind:show="true" />
  <MyComp :show=" true " />
  <MyComp :show='true' />
  <MyComp :foo-bar="true" other />
  <my-comp :show="true"></my-comp>
  <UI.Button :show="true" />
  <component :is="comp" :show="true" />
  <card :show="true" />
  <tr is="vue:my-row" :show="true"></tr>
  <MyComp :a="true" :b="true" />
</template>
