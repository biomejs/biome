---
"@biomejs/biome": patch
---

Added four new style rules to encourage using the nullish coalescing operator (`??`) instead of the logical OR operator (`||`) when providing default values, addressing [#8043](https://github.com/biomejs/biome/issues/8043):

- Added the rule [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) to enforce using `??` instead of `||` for default values.
- Added the rule [`useNullishCoalescingInTernary`](https://biomejs.dev/linter/rules/use-nullish-coalescing-in-ternary/) to simplify ternary expressions that check for null/undefined.
- Added the rule [`useNullishCoalescingAssignment`](https://biomejs.dev/linter/rules/use-nullish-coalescing-assignment/) to prefer `??=` over `||=` for assignments.
- Added the rule [`useIfAsNullishCoalescingAssignment`](https://biomejs.dev/linter/rules/use-if-as-nullish-coalescing-assignment/) to convert if-statement patterns to nullish coalescing assignments.

These rules provide migration support from TypeScript ESLint's `prefer-nullish-coalescing` rule.

The `??` operator only checks for `null` and `undefined`, while `||` checks for any falsy value including `0`, `''`, and `false`. Using `??` can prevent bugs where legitimate falsy values are incorrectly treated as missing.

**Invalid:**

```js
const value = count || 0;
```

**Valid:**

```js
const value = count ?? 0;
```
