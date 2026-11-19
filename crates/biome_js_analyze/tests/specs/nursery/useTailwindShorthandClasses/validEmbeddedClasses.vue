<!-- should not generate diagnostics -->
<template>
  <div :class="'w-4 h-4' === state ? 'size-4' : 'p-2'" />
  <div :class="'w-4 h-4' && 'size-4'" />
  <div :class="{ 'size-4': 'w-4 h-4' }" />
  <div :class="{ 'size-4': active ? 'w-4 h-4' : 'px-2 py-2' }" />
  <div :class="lookup['w-4 h-4']" />
  <div :class="lookup('w-4 h-4')" />
  <div :class="lookup(`w-4 h-4`)" />
  <div :class="(() => 'w-4 h-4')()" />
  <div :class="`size-4 ${state === 'w-4 h-4' ? 'p-2' : 'p-3'}`" />
  <div :class="('w-4 h-4', 'size-4')" />
  <div :id="'w-4 h-4'" />
  <div @class="'w-4 h-4'" />
  <div v-if="'w-4 h-4'" />
  <div :[attribute]="'w-4 h-4'" />
  {{ 'w-4 h-4' }}
  <div :class="[...'w-4 h-4']" />
  <div :class="{...['w-4 h-4']}" />
  <div :class="clsx(...'w-4 h-4')" />
  <div :class="`${{ 'w-4 h-4': active }}`" />
  <div :class="({ 'w-4 h-4': active }) + ''" />
  <div :class="{ [({ 'w-4 h-4': active })]: true }" />
  <div :class="`${['w-4 h-4', 'p-2']}`" />
  <div :class="cached -= 'w-4 h-4'" />
  <div :class="cached['w-4 h-4'] = 'size-4'" />
</template>
