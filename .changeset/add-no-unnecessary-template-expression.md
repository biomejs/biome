---
"@biomejs/biome": patch
---

Added the nursery rule [`noUnnecessaryTemplateExpression`](https://biomejs.dev/linter/rules/no-unnecessary-template-expression/), which disallows template literals that only contain string literal expressions. These can be replaced with a simpler string literal.

For example, the following code triggers the rule:

```js
const a = `${'hello'}`;          // can be 'hello'
const b = `${'prefix'}_suffix`;  // can be 'prefix_suffix'
const c = `${'a'}${'b'}`;        // can be 'ab'
```
