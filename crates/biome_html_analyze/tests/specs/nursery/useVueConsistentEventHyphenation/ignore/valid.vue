<!-- should not generate diagnostics -->
<template>
  <VueComponent @customEvent="onEvent" />
  <VueComponent v-on:customEvent="onEvent" />
  <IgnoredComponent @otherEvent="onEvent" />
  <ignored-component @otherEvent="onEvent" />
</template>
