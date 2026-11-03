<!-- should generate diagnostics -->
<template>
  <slot name="fooBar" />
  <slot name="foo_bar" />
  <slot name="foo:bar" />
  <slot name="foo-Bar_baz" />
  <slot name="foo--bar" />
  <slot name="-foo" />
</template>
