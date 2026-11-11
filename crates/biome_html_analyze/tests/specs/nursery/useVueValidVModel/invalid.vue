<!-- should generate diagnostics -->
<template>
  <div v-model="foo"></div>
  <svg v-model="foo"></svg>
  <keep-alive v-model="foo"></keep-alive>
  <KeepAlive v-model="foo"></KeepAlive>
  <transition v-model="foo"></transition>
  <TransitionGroup v-model="foo"></TransitionGroup>
  <slot v-model="foo"></slot>
  <input type="file" v-model="foo">
  <input v-model:aaa="foo">
  <select v-model:[name]="foo"></select>
  <input v-model.aaa="foo">
  <textarea v-model.lazy.aaa.bbb="foo"></textarea>
  <input v-model>
  <MyComponent v-model="" />
  <MyComponent v-model="   " />
  <input v-model="a + b">
  <input v-model=" a + b ">
  <input v-model="foo()">
  <input v-model="'foo'">
  <input v-model="foo?.()">
  <input v-model="foo?.bar">
  <input v-model="foo?.bar.baz">
  <input v-model="(a?.b)?.c">
  <input v-model="(a?.b).c">
  <input v-model="(null).foo">
  <input v-model="(a?.b).c.d">
  <div v-for="x in list"><input v-model="x"></div>
  <div v-for="x in list"><input v-model="(x)"></div>
  <div v-for="x in list"><input v-model="(((x)))"></div>
  <div v-for="(item, index) in list"><input v-model="index"></div>
  <div v-for="{ name } in list"><input v-model="name"></div>
  <input v-for="x in list" v-model="x">
  <MyComponent v-slot="{ item }"><input v-model="item"></MyComponent>
  <MyComponent><template #default="{ item }"><input v-model="item"></template></MyComponent>
</template>
