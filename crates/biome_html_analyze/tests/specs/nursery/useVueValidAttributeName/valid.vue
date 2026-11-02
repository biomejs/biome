<!-- should not generate diagnostics -->
<template>
  <p foo></p>
  <p foo="bar"></p>
  <p foo-bar></p>
  <p _foo-bar></p>
  <p :foo-bar></p>
  <p xlink:href="value"></p>
  <a :href="url"></a>
  <div v-bind:class="{ active: isActive }"></div>
  <p v-if="seen">Now you see me</p>
  <a v-on:[eventName]="doSomething"></a>
  <form v-on:submit.prevent="onSubmit"></form>
  <a @[event]="doSomething"></a>
  <p @0abc="doSomething"></p>
  <input v-model:0abc="value" />
  <div v-bind="attributes"></div>
  <div :[name]="value"></div>
  <div v-bind:[name]="value"></div>
  <div v-0abc="value"></div>
  <MyComponent 0abc="foo" />
  <MyComponent :0abc="foo" />
  <my-component 0abc="foo"></my-component>
  <component :is="tag" 0abc="foo"></component>
  <tr is="vue:my-row" 0abc="foo"></tr>
</template>
