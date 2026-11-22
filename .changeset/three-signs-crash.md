---
"@biomejs/biome": patch
---

Fixed [#12349](https://github.com/biomejs/biome/issues/12349): Biome now reads quote character references, such as `&quot;` and `&#39;`, as string quotes in the CSS of HTML and SVG `style` attributes. Valid values such as `font-family: &quot;Rotis Sans&quot;` no longer produce CSS parse diagnostics.
