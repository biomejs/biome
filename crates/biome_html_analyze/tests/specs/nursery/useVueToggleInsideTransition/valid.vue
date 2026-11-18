<!-- should not generate diagnostics -->
<template>
  <transition><div v-if="show" /></transition>
  <transition><div v-show="show" /></transition>
  <Transition><div v-if="show" /></Transition>
  <Transition><div v-show="show" /></Transition>
  <Transition><MyComp /></Transition>
  <Transition><my-comp /></Transition>
  <Transition><component :is="component" /></Transition>
  <Transition><div :is="component" /></Transition>
  <Transition><div is="vue:my-comp" /></Transition>
  <Transition><router-view /></Transition>
  <Transition><component /></Transition>
  <Transition><suspense><div /></suspense></Transition>
  <svg height="100" width="100"><transition><circle v-if="show" /></transition></svg>
  <svg height="100" width="100"><transition><MyComponent /></transition></svg>
  <transition><template v-if="show"><div /></template></transition>
  <transition><slot /></transition>
  <transition><div :key="k" /></transition>
  <transition><div v-bind:key="k" /></transition>
  <transition appear><div /></transition>
  <transition :appear="foo"><div /></transition>
  <transition :appear="true"><div /></transition>
  <transition v-bind:appear="true"><div /></transition>
  <transition><div v-if="show" /><div /></transition>
  <transition><div v-if="show"><span /></div></transition>
  <transition></transition>
  <transition>text</transition>
  <transition-group><div /></transition-group>
  <TransitionGroup><div /></TransitionGroup>
  <div><span /></div>
</template>
