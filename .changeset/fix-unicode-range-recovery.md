---
"@biomejs/biome": patch
---

Improved CSS parser recovery for invalid `unicode-range` values that mix wildcard ranges with range intervals. For example, Biome now reports clearer diagnostics for invalid syntax like:

```css
unicode-range: U+11???-2??;
unicode-range: U+11???-;
```

with diagnostics such as:

```text
× Wildcard ranges cannot be combined with a range interval.
  > unicode-range: U+11???-2??;
                            ^

× Expected a codepoint but instead found ';'.
  > unicode-range: U+11???-;
                             ^
```
