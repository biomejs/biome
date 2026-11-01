<!-- should generate diagnostics -->
<template>
  <div :class="['bg-green-500', selected && 'hover:float-left']" />
  <div v-bind:class="{ 'dark:p-4': active }" />
  <div :class="`bg-green-500 ${active ? 'w-1/2' : ''}`" />
</template>
