<!-- should not generate diagnostics -->
<template>
  <button @click="count++">Increment</button>
  <button v-on:click="count++">Increment</button>
</template>
