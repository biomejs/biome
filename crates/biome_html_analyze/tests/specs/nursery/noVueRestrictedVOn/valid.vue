<!-- should not generate diagnostics -->
<template>
  <input @focusin="onFocus" />
  <form v-on:submit="save"></form>
  <div v-on="handlers"></div>
</template>
