---
"@biomejs/biome": patch
---

Fixed [#7470](https://github.com/biomejs/biome/issues/7470): solved a false positive for [`noDuplicateProperties`](https://biomejs.dev/linter/rules/no-duplicate-properties/). Previously, declarations in `@container` and `@starting-style` at-rules were incorrectly flagged as duplicates of identical declarations at the root selector.

For example, the linter no longer flags the `display` declaration in `@container` or the `opacity` declaration in `@starting-style`.

```css
a {
    display: block;
    @container (min-width: 600px) {
        display: none;
    }
}

[popover]:popover-open {
    opacity: 1;
    @starting-style {
        opacity: 0;
    }
}
```
