---
"@biomejs/biome": patch
---

Added the nursery rule [`noSelfNestedLayer`](https://biomejs.dev/linter/rules/no-self-nested-layer/), which disallows nesting a cascade layer inside a layer with the same name. Layer names inside an `@layer` block are relative to it, so the inner layer below creates the sublayer `components.components` instead of adding to `components`.

```css
@layer components {
  @layer components {
    .button {
      color: red;
    }
  }
}
```
