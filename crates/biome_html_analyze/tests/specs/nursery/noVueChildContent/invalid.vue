<!-- should generate diagnostics -->
<template>
  <div v-html="foo">bar</div>
  <div v-html="foo">{{ bar }}</div>
  <div v-html="foo"><span /></div>
  <div v-html="foo"><!-- bar --></div>
  <div v-html="foo">
    <!-- this is a test -->
    <span>foo</span>
    bar
  </div>
  <div v-text="foo">bar</div>
  <p class="a" v-text="foo" v-html="bar">baz</p>
  <MyComponent v-text="foo"><template #default>bar</template></MyComponent>
  <div v-html="foo">bar <!-- trailing --></div>
  <div v-html="foo"> <!-- leading --> bar</div>
</template>
