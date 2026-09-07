<!-- should generate diagnostics -->
<template>
  <RouterView v-slot="{ Component }" :fallback="Component">
    <component :is="Component" :missing="missing" />
  </RouterView>

  <!-- `Component` is only provided inside the RouterView above -->
  <span>{{ Component }}</span>
</template>
