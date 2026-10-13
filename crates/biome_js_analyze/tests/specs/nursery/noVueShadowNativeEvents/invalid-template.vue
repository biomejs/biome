<!-- should generate diagnostics -->
<template>
  <button @click="$emit('click')">Click</button>
  <input @input="$emit(`input`, $event)" />
  <div @keydown="() => $emit('keydown')"></div>
</template>
