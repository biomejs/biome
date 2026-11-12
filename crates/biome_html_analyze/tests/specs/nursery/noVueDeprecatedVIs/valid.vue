<!-- should not generate diagnostics -->
<template>
  <div is="vue:MyComponent"></div>
  <component is="MyComponent" />
  <component :is="componentName" />
  <tr is="vue:my-row"></tr>
  <div v-if="isVisible"></div>
  <div v-istyle="value"></div>
</template>
