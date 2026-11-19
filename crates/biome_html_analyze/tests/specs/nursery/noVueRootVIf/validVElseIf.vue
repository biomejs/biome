<!-- should not generate diagnostics -->
<template>
  <c1 v-if="1" />
  <c2 v-else-if="1" />
  <c3 v-else />
</template>
