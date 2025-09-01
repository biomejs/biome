<script lang="ts" setup>
import { ref } from 'vue'

interface Foo {}

type Bar = 1

export type FooBar = Foo & Bar

debugger

defineProps({})
const emit = defineEmits([])

const count = ref(0)
</script>

<template><div /></template>
