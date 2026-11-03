<!-- should generate diagnostics -->
<template>
  <slot name="foo-bar" />
  <slot name="foo_bar" />
  <slot name="foo:bar" />
  <slot name="FooBar" />
  <slot name="foo-Bar_baz" />
  <slot name='foo-bar'></slot>
  <slot name="foo bar" />
</template>
