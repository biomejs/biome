<!-- should not generate diagnostics -->
<template>
  <input :ref="(element) => (foo = element)" />
  <input id="foo" />
</template>
