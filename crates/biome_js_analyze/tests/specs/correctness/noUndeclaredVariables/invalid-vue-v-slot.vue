<!-- should generate diagnostics -->
<template>
  <RouterView v-slot="{ Component }">
    <component :is="Component" :missing="missing" />
  </RouterView>
</template>
