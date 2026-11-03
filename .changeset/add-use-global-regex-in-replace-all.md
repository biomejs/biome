---
"@biomejs/biome": patch
---

Added the nursery rule [`useGlobalRegexInReplaceAll`](https://biomejs.dev/linter/rules/use-global-regex-in-replace-all/), which reports regular expressions without the `g` flag passed to `replaceAll()`. These calls throw a `TypeError` instead of replacing text.

```js
withSpaces.replaceAll(/\s+/, ",");
```
