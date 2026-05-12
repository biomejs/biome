<!-- should not generate diagnostics -->
<template>
  <input v-on:keyup.enter="submit" />
  <input @keyup.esc="cancel" />
  <input @click.once="submit" />
  <input v-bind:value="value" />
</template>
