<!-- should not generate diagnostics -->
<template>
  <!-- biome-ignore lint/nursery/useVueConsistentEventHyphenation: This third-party component emits camelCase events. -->
  <VueComponent @customEvent="onEvent" />
</template>
