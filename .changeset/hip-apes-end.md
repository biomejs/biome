---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueUndeclaredDirectives`](https://biomejs.dev/linter/rules/no-vue-undeclared-directives/), which reports custom Vue directives that are not declared by a `<script setup>` binding, the component's `directives` option, or the rule's `globals` option. Closes [#11478](https://github.com/biomejs/biome/issues/11478).

```vue
<template>
  <!-- v-highlight is not declared anywhere -->
  <div v-highlight></div>
</template>
```

Aliased named imports in single-file components are now tracked under their local name, so `noUndeclaredVariables` recognizes `vHighlight` in `import { highlight as vHighlight } from "./directives"`.
