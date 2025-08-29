---
"@biomejs/biome": minor
---

Added the new lint rule `useConsistentArrowReturn`.

This rule enforces a consistent return style for arrow functions.

### Invalid

```js
const f = () => {
    return 1;
}
```

This rule is a port of ESLint's [arrow-body-style](https://eslint.org/docs/latest/rules/arrow-body-style) rule.
