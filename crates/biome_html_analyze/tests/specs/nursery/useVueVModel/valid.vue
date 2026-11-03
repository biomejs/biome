<!-- should not generate diagnostics -->
<template>
  <my-component v-model="foo" />
  <my-component v-model:foo="bar" />
  <my-component :foo="bar" @update:foo="baz = $event" />
  <my-component :foo="bar" @update:foo="updateFoo($event)" />
  <my-component :foo="bar" @update:foo="(val) => updateFoo(val)" />
  <my-component :foo="bar" />
  <my-component @update:foo="bar = $event" />
  <input :value="bar" @update:value="bar = $event" />
  <div :foo="bar" @update:foo="bar = $event"></div>
  <my-component :foo="bar" @update:foo.once="bar = $event" />
  <my-component :foo.sync="bar" @update:foo="bar = $event" />
  <my-component :foo="bar" @update:foo="bar = $event; doSomething()" />
  <my-component :foo="bar" @update:foo="bar += $event" />
  <my-component :foo="bar" @update:foo="(val) => bar += val" />
  <my-component :foo="bar" @update:foo="(val) => { bar = val; doSomething() }" />
  <my-component :foo="bar" @update:foo="(val, extra) => bar = val" />
  <my-component :foo="bar" @update:foo="(val) => bar = $event" />
  <my-component :foo="bar" @update:foo="bar = other" />
  <my-component :[prop]="bar" @update:foo="bar = $event" />
  <my-component :foo="bar" @[event]="bar = $event" />
  <my-component :foo="bar" @update:bar="bar = $event" />
  <my-component :foo="bar" @foo="bar = $event" />
  <my-component :foo="obj.bar" @update:foo="obj.baz = $event" />
  <my-component :foo="bar" @update:foo />
</template>
