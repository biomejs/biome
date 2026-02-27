<!-- Vue valid cases - should NOT trigger the rule -->

<template>
  <a href="/about">About</a>
  <a href="https://example.com">External</a>
  <a href="#hash">Hash</a>
  <!-- Not an anchor tag -->
  <span href="javascript:void(0)">Not a link</span>
</template>
