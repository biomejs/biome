---
"@biomejs/biome": patch
---

Added the nursery rule [`noSvelteInternal`](https://biomejs.dev/linter/rules/no-svelte-internal/), which disallows importing from `svelte/internal` and its submodules. These modules are private to Svelte and can change or disappear in any release.

```js
import { get_current_component } from "svelte/internal";
```
