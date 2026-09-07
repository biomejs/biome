---
"@biomejs/biome": patch
---

Fixed [#11478](https://github.com/biomejs/biome/issues/11478): [`noUndeclaredVariables`](https://biomejs.dev/linter/rules/no-undeclared-variables) now reports undeclared custom Vue directives, recognizing `<script setup>` bindings, statically resolvable `directives` options, and configured globals.

```vue
<template>
  <!-- v-highlight is not declared anywhere -->
  <div v-highlight></div>
</template>
```

Aliased named imports in single-file components are now tracked under their local name, so `noUndeclaredVariables` recognizes `vHighlight` in `import { highlight as vHighlight } from "./directives"`.
