---
"@biomejs/biome": patch
---

Added the new nursery rule [`noNestedTemplateLiterals`](https://biomejs.dev/linter/rules/no-nested-template-literals/), which disallows template literals written inside other template literals.

```js
const message = `I have ${color ? `${count} ${color}` : count} apples`;
```
