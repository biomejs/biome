<!-- should not generate diagnostics -->
<template>
  <MyComp v-if="true" />
  <MyComp v-bind="true" />
  <MyComp v-loading="true" />
  <MyComp show="true" />
  <MyComp :show="value" />
  <MyComp v-bind:show="value" />
  <MyComp :show="false" />
  <MyComp v-bind:show="false" />
  <MyComp :show="'true'" />
  <MyComp :show="true && value" />
  <MyComp :show />
  <MyComp show />
  <MyComp :[name]="true" />
  <MyComp :show.prop="true" />
  <MyComp v-model:show="true" />
  <input :checked="true" />
  <input v-bind:checked="true" />
  <input checked="checked" />
  <button is="my-button" :disabled="true"></button>
  <math><mo :stretchy="true">+</mo></math>
</template>
