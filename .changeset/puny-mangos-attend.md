---
"@biomejs/biome": patch
---

Added the nursery rule [`useConsistentUnicodeBom`](https://biomejs.dev/linter/rules/use-consistent-unicode-bom/), which disallows a leading Unicode byte order mark by default and can require one with `bom: "always"`. The rule supports JavaScript, TypeScript, JSON, CSS, GraphQL, HTML, and Markdown, provides a fix, and preserves the `unicode-bom` option during ESLint migration.

The following example is invalid by default because it starts with an invisible U+FEFF character:

```js
﻿const value = 1;
```
