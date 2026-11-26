<!-- should not generate diagnostics -->
<template>
  <template id="a">...</template>
  <template :id="a">...</template>
  <template v-bind:id="a">...</template>
  <template ref="b">...</template>
  <template :ref="b">...</template>
  <template class="b" ref="c"/>
  <template ID="a">...</template>
</template>
