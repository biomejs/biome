<!-- should generate diagnostics -->
<template>
  <div :class="['w-4 h-4', selected && 'px-2 py-2']" />
  <div v-bind:class="{ 'mr-3 ml-3': active }" />
  <div :class="active ? `w-4 h-4` : 'px-2 py-2'" />
  <div :class="['w-4 h-4', [active && 'px-2 py-2']]" />
  <div :class="{ ['mr-3 ml-3']: active }" />
  <div :class="`w-4 h-4 ${active ? 'px-2 py-2' : 'mr-3 ml-3'}`" />
  <div :class="classes || 'w-4 h-4'" />
  <div :class="classes ?? 'px-2 py-2'" />
  <div :class="(ignored, 'w-4 h-4')" />
  <div :class="'w-4 h-4 ' + extra" />
  <div :class="[...['w-4 h-4']]" />
  <div :class="{...{'px-2 py-2': active}}" />
  <div :class="cached = 'w-4 h-4'" />
  <div :class="cached += ' w-4 h-4'" />
  <div :class="cached ??= 'w-4 h-4'" />
</template>
