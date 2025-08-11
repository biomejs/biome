---
"@biomejs/biome": patch
---

Fixed [#7152](https://github.com/biomejs/biome/issues/7152). Now the rule `noDuplicateFontNames` correctly detects font names with spaces e.g. `Liberation Mono`. The diagnostic of the rule now points to the first instances of the repeated font.

The following example doesn't trigger the rule anymore:

```css
c { font-family: SF Mono, Liberation Mono, sans-serif; }
d { font: 1em SF Mono, Liberation Mono, sans-serif; }
```
