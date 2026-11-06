---
"@biomejs/biome": patch
---

Fixed [#12349](https://github.com/biomejs/biome/issues/12349): quote character references in HTML and SVG `style` attributes are now decoded before embedded CSS parsing. Valid values such as `font-family: &quot;Rotis Sans&quot;` no longer produce CSS parse diagnostics.
