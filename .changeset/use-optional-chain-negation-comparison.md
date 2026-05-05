---
"@biomejs/biome": patch
---

Fixed [#10244](https://github.com/biomejs/biome/issues/10244): [`useOptionalChain`](https://biomejs.dev/linter/rules/use-optional-chain/) now flags the negation + comparison form on the right-hand side of `||`. The rule already handled `!foo || !foo.bar`; it now also handles cases where the rightmost operand is a binary comparison (`==`, `!=`, `===`, `!==`, `<`, `<=`, `>`, `>=`) whose left side extends the chain.

For example, the following are now reported and offer an unsafe fix:

```js
!foo || foo.bar !== "x";
!foo || foo.bar.baz === undefined;
!foo || !foo.bar || foo.bar.baz > 0;
```

The fix removes the leading `!`-negated operands and inserts `?.` into the comparison's left side: `foo?.bar !== "x"`, `foo?.bar?.baz === undefined`, `foo?.bar?.baz > 0`.
