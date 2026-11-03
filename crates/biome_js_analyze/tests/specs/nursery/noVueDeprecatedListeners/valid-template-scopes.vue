<!-- should not generate diagnostics -->
<template>
  <div v-on="() => {
    function click ($listeners) {
      fn(foo.$listeners)
      fn($listeners)
    }
  }" />
  <VueComp>
    <template v-slot="{ $listeners }">
      <div v-on="$listeners" />
    </template>
  </VueComp>
</template>
