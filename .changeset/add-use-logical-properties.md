---
"@biomejs/biome": patch
---

Enhanced the nursery rule [`useLogicalProperties`](https://biomejs.dev/linter/rules/use-logical-properties/) to check physical CSS values in addition to physical property names.

The rule recognizes physical values in `frame-sizing`, `anchor-size()`, and `anchor()`, with direction-aware replacements controlled by the existing `direction` option.

This is a second part of the rule covering parts of [#9034](https://github.com/biomejs/biome/issues/9034)
