<!-- should generate diagnostics -->
<template>
  <input v-on:keyup.13="submit" />
  <input @keyup.27="cancel" />
  <input @keydown.27="cancel" />
  <input @keyup.13.exact="submit" />
</template>
