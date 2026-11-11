<!-- should generate diagnostics -->
<template>
  <RouterLink tag="div" to="/">Home</RouterLink>
  <router-link :tag="someVariable" to="/">Home</router-link>
  <NuxtLink tag="div" to="/">Home</NuxtLink>
  <nuxt-link tag="div" to="/">Home</nuxt-link>
  <NuxtLink :tag="someVariable" to="/">Home</NuxtLink>
  <nuxt-link :tag="someVariable" to="/">Home</nuxt-link>
  <UILink tag="li" to="/">Home</UILink>
  <u-i-link tag="li" to="/">Home</u-i-link>
</template>
