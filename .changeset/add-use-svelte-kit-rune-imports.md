---
"@biomejs/biome": patch
---

Added the nursery rule [`useSvelteKitRuneImports`](https://biomejs.dev/linter/rules/use-svelte-kit-rune-imports/), which reports imports from the deprecated `$app/stores` module and suggests `$app/state` instead.

```js
import { page } from "$app/stores";
```
