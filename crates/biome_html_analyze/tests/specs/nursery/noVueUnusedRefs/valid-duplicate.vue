<!-- should not generate diagnostics -->
<template>
  <div v-if="ok" ref="box" />
  <div v-else ref="box" />
  <button @click="$refs.box.focus()">Focus</button>
</template>
