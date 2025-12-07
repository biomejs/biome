<!-- should generate diagnostics -->

<template>
  <!-- v-else with argument -->
  <div v-else:arg></div>

  <!-- v-else with modifier -->
  <div v-else.mod></div>

  <!-- v-else with value -->
  <div v-else="value"></div>

  <!-- v-else without previous sibling having v-if/v-else-if -->
  <div v-else></div>

  <!-- v-else after element without v-if/v-else-if -->
  <div></div>
  <div v-else></div>

  <!-- v-else on same element as v-if -->
  <div v-if="condition" v-else></div>

  <!-- v-else on same element as v-else-if -->
  <div v-else-if="condition" v-else></div>

  <!-- v-else with whitespace between but previous element lacks v-if -->
  <div>Some content</div>
  
  <div v-else></div>
</template>