<!-- should not generate diagnostics -->
<script setup>
// biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette.
const classes = clsx("bg-pink-500");
</script>
<template>
<!-- biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette. -->
<div class="bg-pink-500" />

<div :class="/* biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette. */
'bg-pink-500'" />

<div v-bind:class="/* biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette. */
['bg-pink-500', active && 'text-red-500']" />

<div :class="/* biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette. */
{ 'bg-pink-500': active }" />

<div :class="/* biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette. */
active ? 'bg-pink-500' : 'text-red-500'" />

<div :class="/* biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette. */
`bg-pink-500 ${extra}`" />

<div :class="/* biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette. */
clsx('bg-pink-500', { 'text-red-500': active })" />
</template>
