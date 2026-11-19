<!-- should not generate diagnostics -->
<template>
  <button @click="foo" />
  <template v-for="e in list">
    <button @click="e" />
    <button @click="foo(e)" />
    <button @click="foo(e, $event)" />
  </template>
</template>
