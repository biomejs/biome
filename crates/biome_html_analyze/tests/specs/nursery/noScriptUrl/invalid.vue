<!-- Vue invalid cases - should trigger the rule -->

<template>
  <a href="javascript:void(0)">Void</a>
  <a href=" javascript:alert('XSS') ">Alert</a>
  <a href="JAVASCRIPT:doSomething()">Uppercase</a>
</template>
