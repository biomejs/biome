<!-- should not generate diagnostics -->
<template>
  <div :id="'\n'" />
  <div :id="'\r'" />
  <div :id="`\x41`" />
</template>
