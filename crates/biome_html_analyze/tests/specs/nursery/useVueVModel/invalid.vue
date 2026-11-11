<!-- should generate diagnostics -->
<template>
  <my-component :modelValue="foo" @update:modelValue="foo = $event" />
  <my-component :model-value="foo" @update:model-value="foo = $event" />
  <my-component :model-value="foo" @update:modelValue="foo = $event" />
  <MyComponent :foo="bar" @update:foo="bar = $event" />
  <my-component
    :foo="bar"
    bar
    @update:foo="bar = $event"
  />
  <my-component v-bind:foo='bar["baz"]' v-on:update:foo='bar["baz"] = $event' />
  <my-component :foo="bar" @update:foo="(val) => bar = val" />
  <my-component :foo="bar" @update:foo="val => bar = val" />
  <my-component :foo="obj.bar" @update:foo="obj.bar = $event" />
  <my-component :foo="obj.bar" @update:foo="(val) => obj.bar = val" />
  <my-component :foo="a" @update:foo="a = $event" :bar="b" @update:bar="b = $event" />
  <my-component @update:foo="bar = $event" :foo="bar" />
  <my-component :foo-bar="baz" @update:fooBar="baz = $event" />
  <my-component :foo="bar" @update:foo="bar = $event"></my-component>
  <my-component :foo="bar" bar @update:foo="bar = $event"></my-component>
  <my-component :foo="bar" @update:foo="bar = $event"/>
  <component :is="view" :foo="bar" @update:foo="bar = $event" />
  <input is="vue:my-input" :value="bar" @update:value="bar = $event" />
  <card :foo="bar" @update:foo="bar = $event" />
</template>
