---
"@biomejs/biome": patch
---

Fixed [`noUndeclaredCustomProperties`](https://biomejs.dev/linter/rules/no-undeclared-custom-properties/) reporting custom properties declared in SCSS mixin bodies or with interpolated names. A declaration such as `--color-#{$name}` now counts as declaring every custom property that starts with `--color-`, and `--#{$name}` as declaring every custom property.

```scss
@mixin spacing-variables {
  --spacing: 4px;
}

:root {
  @include spacing-variables;

  @each $name in primary, secondary {
    --color-#{$name}: red;
  }
}

.a {
  margin: var(--spacing);
  color: var(--color-primary);
}
```
