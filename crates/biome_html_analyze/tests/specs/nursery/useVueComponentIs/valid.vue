<!-- should not generate diagnostics -->
<template>
  <component :is="currentView" />
  <component v-bind:is="currentView" />
  <component :is="currentView"></component>
  <component :is.prop="currentView" />
  <component class="wrapper" :is="currentView" />
  <component v-bind="props" :is="currentView" />
  <Component :is="currentView" />
  <div :is="currentView" />
  <div is="vue:MyComponent" />
  <MyComponent />
  <COMPONENT />
  <components />
  <custom-component />
</template>
