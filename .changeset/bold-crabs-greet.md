---
"@biomejs/biome": patch
---

Added the nursery rule [`useCapitalizedConstructors`](https://biomejs.dev/linter/rules/use-capitalized-constructors/), a port of ESLint's `new-cap`. It reports `new` expressions whose callee name starts with a lowercase letter, and calls without `new` whose callee name starts with an uppercase letter.

```js
const friend = new person();
const other = Person();
```
