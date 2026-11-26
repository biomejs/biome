<!-- should not generate diagnostics -->
<template>
  <div v-for="x of xs"></div>
  <div v-for="(x, index) of xs"></div>
  <div v-for="x    of    xs"></div>
</template>
