<script lang="ts" setup>
import { ref } from 'vue'

interface Foo {}

type Bar = 1

export type FooBar = Foo & Bar

debugger

// Define the props and emits
const emit = defineEmits([])
/** Props */
defineProps({})

definePage({})

const count = ref(0)
</script>

<template><div /></template>
