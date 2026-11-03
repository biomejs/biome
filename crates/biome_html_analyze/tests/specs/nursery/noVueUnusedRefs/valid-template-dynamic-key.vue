<!-- should not generate diagnostics -->
<template>
  <input ref="x" />
  <input ref="y" />
  <button @click="alert($refs[foo].value)">Click</button>
</template>
