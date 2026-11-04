---
"@biomejs/biome": patch
---

Added the new nursery rule [`noScssNullComparison`](https://biomejs.dev/linter/rules/no-scss-null-comparison/). It reports comparisons to `null` in Sass `@if` and `@else if` conditions, because `null` already counts as false in a condition.

```scss
a {
  @if $value == null {}
}
```
