<!-- should not generate diagnostics -->
<template>
  <div :foo="bar" />
  <DialogTrigger data-slot="dialog-trigger" v-bind="props">
    <slot></slot>
  </DialogTrigger>
</template>
