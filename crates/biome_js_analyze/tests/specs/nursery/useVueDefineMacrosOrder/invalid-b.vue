<script lang="ts" setup>
import { ref } from 'vue'

interface Foo {}

type Bar = 1

export type FooBar = Foo & Bar

debugger

const count = ref(0)

const emit = defineEmits([])
</script>

<template><div /></template>
