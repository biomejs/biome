<!-- should generate diagnostics -->
<template>
  <div v-if="(a)" />
  <div v-else-if="a" />

  <div v-if="a" />
  <!-- a comment between branches -->
  <div v-else-if="a" />

  <template v-if="user.isAdmin">
    <p>Admin</p>
  </template>
  <template v-else-if="user.isAdmin">
    <p>Also admin</p>
  </template>

  <MyComponent v-if="a ?? b" />
  <MyComponent v-else-if="a ?? b" />

  <div v-if="a" />
  <div v-else-if="(b && a)" />

  <div v-if="(a || b) && (c || d)" />
  <div v-else-if="(d || c) && (b || a)" />

  <div v-if="a" />
  <div v-else-if="b" />
  <div v-else-if="a" />
  <div v-else>fallback</div>
</template>
