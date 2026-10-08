---
"@biomejs/biome": patch
---

Added the new nursery rule [`noConsoleSpaces`](https://biomejs.dev/linter/rules/no-console-spaces/), which reports a leading or trailing space in a `console.log()`, `console.debug()`, `console.info()`, `console.warn()`, or `console.error()` argument that sits next to another argument. These methods already put a space between arguments, so such a space prints twice.

```js
console.log("abc ", "def");
```
