<!-- should generate diagnostics -->
<template>
  <p 0abc></p>
  <p -def></p>
  <p 0abc="value"></p>
  <p a$b></p>
  <p a#b="value"></p>
  <p a&b></p>
  <p v-bind:0abc="value"></p>
  <p v-bind:-def.prop="value"></p>
  <p :0abc="value"></p>
  <p :-def="value"></p>
  <p :0abc.camel="value"></p>
  <svg 0abc></svg>
  <div is="my-button" 0abc></div>
  <div><span foo 0abc bar></span></div>
</template>
