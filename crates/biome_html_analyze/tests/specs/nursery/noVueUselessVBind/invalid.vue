<!-- should generate diagnostics -->
<template>
  <div :id="'foo'" />
  <div v-bind:id="'foo'" />
  <div :id="`foo`" />
  <div :id='"foo"' />
  <div :id="('foo')" />
  <div :id="''" />
  <div :id="    'foo'    " />
  <div :id='    "foo"    ' />
  <div :id="' \'foo\' '" />
  <div :id='" \"foo\" "' />
  <div :id="'\\'" />
  <div :id="'\\r'" />
  <div :id="'&quot;'" />
  <div :id='"&apos;"' />
  <div :id="`a\`b\${c}`" />
  <div id="foo" :title="'Hello'" />
  <MyComponent :title="'Hello'" />
</template>
