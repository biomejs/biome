---
"@biomejs/biome": patch
---

Added the nursery rule [`noTripleSlashReference`](https://biomejs.dev/linter/rules/no-triple-slash-reference/), which reports `/// <reference path="..." />` comments and `/// <reference types="..." />` comments for packages that the file also imports. The `lib`, `path`, and `types` options control which kinds of references are allowed.

```ts
/// <reference path="./globals.d.ts" />
```
