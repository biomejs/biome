<!-- should generate diagnostics -->
<template>
  <div :id="'comment'/*comment*/" />
  <div :id="'comment'//comment
  " />
  <div :id="'\n'" />
  <div :id="'\r'" />
  <div :id="'\x41'" />
  <div :id="'\u0041'" />
  <div :id="'\0'" />
  <div :id="'\
'" />
  <div class="foo" :class="'bar'" />
</template>
