<!-- should generate diagnostics -->
<template>
  <RouterLink tag="div" to="/">Home</RouterLink>
  <router-link tag="div" to="/">Home</router-link>
  <RouterLink :tag="someVariable" to="/">Home</RouterLink>
  <router-link :tag="someVariable" to="/">Home</router-link>
  <RouterLink v-bind:tag="someVariable" to="/">Home</RouterLink>
  <RouterLink tag to="/">Home</RouterLink>
  <RouterLink tag="li" to="/" />
  <ul>
    <router-link to="/" tag="li">
      <span>Home</span>
    </router-link>
  </ul>
</template>
