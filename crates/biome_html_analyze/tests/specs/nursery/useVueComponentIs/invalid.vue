<!-- should generate diagnostics -->
<template>
  <component />
  <component></component>
  <component is="MyComponent" />
  <component v-bind="props" />
  <component :="props" />
  <component :[name]="currentView" />
  <component v-bind:[name]="currentView" />
  <component :foo="currentView" />
  <component v-on:is="currentView" />
  <component @is="handler" />
  <component :IS="currentView" />
  <Component />
  <Component></Component>
  <div>
    <component>
      <span>child</span>
    </component>
  </div>
</template>
