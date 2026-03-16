<!-- should not generate diagnostics -->
<script setup lang="ts">
import { Component, FooEnum } from './types';
</script>
<template>
  <Component />
  <div>{{ FooEnum.Foo }}</div>
</template>
