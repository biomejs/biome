---
"@biomejs/biome": minor
---

feat(lint): add `noUnnecessaryTemplateExpression` rule

This rule reports template literals that contain only a single interpolation or string literal,
and where the template literal would be equivalent to a simpler expression.

The rule will automatically fix cases where:
- A template contains only a single interpolation: `` `${expr}` `` → `expr`
- A template contains only string literals that can be combined: `` `${'a'}${'b'}` `` → `'ab'`
- A template literal without interpolations: `` `text` `` → `'text'`

Examples:

```js
// ❌ Invalid
const wrapped = `${value}`;
const combined = `${'Hello '}${'World'}`;
const plain = `just text`;

// ✅ Valid
const greeting = `Hello, ${name}!`;
const multiline = `line 1
line 2`;
const tagged = css`color: red;`;
```

The rule helps simplify code by removing unnecessary template literal syntax when simpler alternatives are available.