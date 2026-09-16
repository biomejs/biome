<!-- should not generate diagnostics -->
<template>
  <button @click="$emit('custom', $event)">Click</button>
  <button v-on:click="$emit('custom', $event)">Click</button>
</template>
