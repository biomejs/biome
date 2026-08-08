---
"@biomejs/biome": patch
---

Fixed [#11215](https://github.com/biomejs/biome/issues/11215): [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) and [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/) no longer report a false positive for bindings referenced only through the shorthand form of the Svelte `class:` and `style:` directives.

```svelte
<script>
import { active } from "./flags";

const color = "red";
</script>

<!-- `active` and `color` are no longer reported as unused. -->
<div class:active style:color>Text</div>
```

Hyphenated properties such as `class:is-active` and `style:background-color` cannot name a binding, so they still register no reference.
