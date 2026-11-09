---
"@biomejs/biome": patch
---

Added the new nursery rule [`useGlobalRegexInMatchAll`](https://biomejs.dev/linter/rules/use-global-regex-in-match-all/), which reports calls to `matchAll()` with a regular expression that doesn't have the `g` flag. These calls throw a `TypeError` at runtime.

```js
text.matchAll(/pattern/);
```
