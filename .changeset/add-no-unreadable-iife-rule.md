---
"@biomejs/biome": patch
---

Added the nursery rule [`noUnreadableIife`](https://biomejs.dev/linter/rules/no-unreadable-iife/), which reports arrow functions that are called right where they are defined and whose body is wrapped in parentheses.

```js
const foo = (bar => (bar ? bar.baz : baz))(getBar());
```
