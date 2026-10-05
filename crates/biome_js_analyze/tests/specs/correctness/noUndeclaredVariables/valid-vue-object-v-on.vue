<!-- should not generate diagnostics -->
<template>
  <button v-on="{ click: () => console.log('clicked') }"></button>
</template>
