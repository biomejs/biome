---
"@biomejs/biome": minor
---

Added support for parsing and formatting the CSS `@function` at-rule from the [CSS Mixins Module Level 1](https://drafts.csswg.org/css-mixins-1/#function-rule) specification. Addresses issue [#8184](https://github.com/biomejs/biome/issues/8184).

```css
@function --transparent(--color <color>, --alpha <number>: 0.5) returns <color> {
  result: oklch(from var(--color) l c h / var(--alpha));
}
```
