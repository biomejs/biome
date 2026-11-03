<!-- should generate diagnostics -->
<template>
  <transition><div /></transition>
  <Transition><div /></Transition>
  <Transition><div>Content</div></Transition>
  <transition><div /><div /></transition>
  <transition><div v-for="e in list" /></transition>
  <svg height="100" width="100"><transition><circle /></transition></svg>
  <transition><template v-for="e in list"><div /></template></transition>
  <Transition>
    text
    <!-- comment -->
    <div />
  </Transition>
  <transition :appear="false"><div /></transition>
  <transition v-bind:appear="false"><div /></transition>
  <transition @appear="isLoaded"><div /></transition>
  <transition><dialog v-dialog="show" /></transition>
  <transition><div key="static" /></transition>
  <transition name="fade" mode="out-in"><p class="message">Saved</p></transition>
</template>
