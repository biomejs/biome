<!-- should not generate diagnostics -->
<template>
  <MyComp :show="true" />
  <MyComp v-bind:show="true" />
  <MyComp show="true" />
  <input checked />
  <MyComp v-else />
  <math><mo stretchy>+</mo></math>
</template>

<i18n global>
{}
</i18n>
