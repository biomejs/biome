---
"@biomejs/biome": patch
---

Fixed [`noUnknownTypeSelector`](https://biomejs.dev/linter/rules/no-unknown-type-selector/) reporting `from` and `to` rules directly inside SCSS mixins and `@include` blocks that Sass places inside `@keyframes`:

```scss
@mixin frames {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes fade {
  @include frames;
}
```
