<script lang="ts" setup>
import { ref } from 'vue'

interface Foo {}

type Bar = 1

export type FooBar = Foo & Bar

debugger

const emit = defineEmits([])
const props = withDefaults(defineProps({}), { a: 1 })

const count = ref(0)
</script>

<template><div /></template>
