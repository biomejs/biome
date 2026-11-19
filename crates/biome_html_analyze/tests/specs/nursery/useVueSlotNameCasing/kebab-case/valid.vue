<!-- should not generate diagnostics -->
<template>
  <slot name="foo" />
  <slot name="foo-bar" />
  <slot name="foo-bar-2" />
  <slot :name="fooBar" />
</template>
