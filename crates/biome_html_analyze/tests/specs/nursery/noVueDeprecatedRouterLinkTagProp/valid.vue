<!-- should not generate diagnostics -->
<template>
  <RouterLink to="/">Home</RouterLink>
  <router-link to="/">Home</router-link>
  <RouterLink to="/">
    <div>Home</div>
  </RouterLink>
  <router-link to="/">
    <div>Home</div>
  </router-link>
  <RouterLink to="/" custom v-slot="{ navigate }">
    <li role="link" @click="navigate">Home</li>
  </RouterLink>
  <RouterLink :[attribute]="value" to="/">Home</RouterLink>
  <RouterLink @tag="handler" to="/">Home</RouterLink>
  <RouterLink data-tag="div" to="/">Home</RouterLink>
  <NuxtLink tag="div" to="/">Home</NuxtLink>
  <nuxt-link tag="div" to="/">Home</nuxt-link>
  <div tag="div"></div>
  <MyRouterLink tag="div" to="/">Home</MyRouterLink>
</template>
