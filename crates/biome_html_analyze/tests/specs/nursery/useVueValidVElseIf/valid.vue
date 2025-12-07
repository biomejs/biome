<!-- should not generate diagnostics -->

<template>
  <!-- Proper chain: v-if -> v-else-if -> v-else-if -> v-else -->
  <div v-if="a"></div><div v-else-if="b"></div><div v-else-if="c"></div><div v-else></div>

  <!-- Multiple chains separated by unrelated markup -->
  <div v-if="a"></div><div v-else-if="b"></div><div v-else></div>
  <span>Unrelated content</span>
  <div v-if="x"></div><div v-else-if="y"></div><div v-else></div>

  <!-- v-if with complex expression followed by v-else-if -->
  <div v-if="user && user.isAdmin"></div><div v-else-if="user && user.isModerator"></div>

  <!-- v-if followed by v-else-if referencing props -->
  <div v-if="isVisible"></div><div v-else-if="props.enabled"></div>

  <!-- Accept preceding element with v-else-if -->
  <div v-if="a"></div><div v-else-if="b"></div><div v-else-if="c"></div>

  <!-- Interspersed whitespace/comment nodes between chain elements -->
  <div v-if="a"></div>
  <!-- comment -->
  <div v-else-if="b"></div>
  
  <!-- another comment -->
  <div v-else-if="c"></div><div v-else></div>

  <!-- Template elements with v-if chains -->
  <template v-if="condition">
    <p>Content</p>
  </template>
  <template v-else-if="otherCondition">
    <span>Other content</span>
  </template>
  <template v-else>
    <div>Default content</div>
  </template>

  <!-- Multiple valid v-else-if in sequence -->
  <div v-if="step === 1"></div>
  <div v-else-if="step === 2"></div>
  <div v-else-if="step === 3"></div>
  <div v-else-if="step === 4"></div>
  <div v-else></div>

  <!-- Complex expressions -->
  <div v-if="user.age >= 18 && user.hasPermission"></div>
  <div v-else-if="user.age >= 13 && user.parentalConsent"></div>
  <div v-else></div>
</template>