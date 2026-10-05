<!-- should generate diagnostics -->
<template>
  <div :class="['bg-pink-500', selected && 'hover:text-red-500/80']" />
  <div v-bind:class="{ 'border-slate-950': active }" />
  <div :class="active ? `bg-pink-500` : 'hover:text-red-500/80'" />
  <div :class="['bg-pink-500', [active && 'hover:text-red-500/80']]" />
  <div :class="{ ['border-slate-950']: active }" />
  <div :class="`bg-pink-500 ${active ? 'hover:text-red-500/80' : 'border-slate-950'}`" />
  <div :class="classes || 'bg-pink-500'" />
  <div :class="classes ?? 'hover:text-red-500/80'" />
  <div :class="(ignored, 'bg-pink-500')" />
  <div :class="'bg-pink-500 ' + extra" />
  <div :class="[...['bg-pink-500']]" />
  <div :class="{...{'hover:text-red-500/80': active}}" />
  <div :class="cached = 'bg-pink-500'" />
  <div :class="cached += ' bg-pink-500'" />
  <div :class="cached ??= 'bg-pink-500'" />
</template>
