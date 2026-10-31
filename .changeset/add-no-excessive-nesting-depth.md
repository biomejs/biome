---
"@biomejs/biome": patch
---

Added the nursery rule [`noExcessiveNestingDepth`](https://biomejs.dev/linter/rules/no-excessive-nesting-depth/), which limits how deeply CSS rules and at-rules can be nested. The maximum depth defaults to `3` and can be changed with the `maxDepth` option, and the `ignoreAtRules` option lists at-rules, such as `media`, that don't add a level.

The following example is invalid because `.e` is nested four levels deep:

```css
.a {
  .b {
    .c {
      .d {
        .e {
          color: red;
        }
      }
    }
  }
}
```
