<!-- should not generate diagnostics -->
<template>
  <div :class="'bg-pink-500' === state ? 'bg-primary' : 'text-white'" />
  <div :class="'bg-pink-500' && 'bg-primary'" />
  <div :class="{ 'bg-primary': 'bg-pink-500' }" />
  <div :class="{ 'bg-primary': active ? 'bg-pink-500' : 'hover:text-red-500/80' }" />
  <div :class="lookup['bg-pink-500']" />
  <div :class="lookup('bg-pink-500')" />
  <div :class="lookup(`bg-pink-500`)" />
  <div :class="(() => 'bg-pink-500')()" />
  <div :class="`bg-primary ${state === 'bg-pink-500' ? 'text-white' : 'border-transparent'}`" />
  <div :class="('bg-pink-500', 'bg-primary')" />
  <div :id="'bg-pink-500'" />
  <div @class="'bg-pink-500'" />
  <div v-if="'bg-pink-500'" />
  <div :[attribute]="'bg-pink-500'" />
  {{ 'bg-pink-500' }}
  <div :class="[...'bg-pink-500']" />
  <div :class="{...['bg-pink-500']}" />
  <div :class="clsx(...'bg-pink-500')" />
  <div :class="`${{ 'bg-pink-500': active }}`" />
  <div :class="({ 'bg-pink-500': active }) + ''" />
  <div :class="{ [({ 'bg-pink-500': active })]: true }" />
  <div :class="`${['bg-pink-500', 'text-white']}`" />
  <div :class="cached -= 'bg-pink-500'" />
  <div :class="cached['bg-pink-500'] = 'bg-primary'" />
  <div :class="['bg-white text-black', active && 'border-transparent fill-current stroke-inherit', 'bg-[#ff00aa]']" />
  <div :class="{ 'bg-primary': active, 'text-white': selected }" />
</template>
