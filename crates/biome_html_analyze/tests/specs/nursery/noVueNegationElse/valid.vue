<!-- should not generate diagnostics -->
<template>
  <div v-if="!foo">Content</div>

  <div v-if="!foo">First</div>
  <div v-if="bar">Second</div>

  <div v-if="!foo">Content</div>
  <div v-else-if="bar">Alternative</div>

  <div v-if="!foo">First</div>
  <div v-else-if="!bar">Second</div>

  <div v-if="a != b">Not equal</div>

  <div v-if="a !== b">Strictly not equal</div>

  <div v-if="a != b">Not equal</div>
  <div v-else-if="c">Alternative</div>

  <div v-if="!foo">First</div>
  <div v-else-if="!bar">Second</div>
  <div v-else-if="baz">Third</div>

  <div v-if="!a">First</div>
  <div v-else-if="b">Second</div>
  <div v-else>Default</div>

  <div v-if="foo">Positive</div>
  <div v-else>Negative</div>

  <div v-if="foo && bar">Both true</div>
  <div v-else>Otherwise</div>

  <div v-if="!foo && bar">Negated operand</div>
  <div v-else>Otherwise</div>

  <div v-if="foo === bar">Equal</div>
  <div v-else>Not equal</div>

  <div v-if="foo">
    <span v-if="!bar">Nested content</span>
  </div>

  <div v-if="!foo">First</div>
  text between the branches
  <div v-else>Second</div>

  <div v-if="!!foo">Double negation</div>
  <div v-else>Otherwise</div>

  <div v-if="!(!foo)">Parenthesized double negation</div>
  <div v-else>Otherwise</div>

  <div v-show="!foo">Shown</div>
  <div v-else>Orphan</div>
</template>
