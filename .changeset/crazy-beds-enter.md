---
"@biomejs/biome": minor
---

The `allowDoubleNegation` option has been added to [`noImplicitCoercions`](https://biomejs.dev/linter/rules/no-implicit-coercions) to allow ignoring double negations inside code.

With the option enabled, the following example is considered valid and is ignored by the rule:

```js
const truthy = !!value;
```
