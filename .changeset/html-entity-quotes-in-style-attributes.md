---
"@biomejs/biome": patch
---

Fixed [#12349](https://github.com/biomejs/biome/issues/12349): Biome no longer reports parse errors for CSS strings quoted with HTML entities, such as `&quot;` or `&#39;`, in `style` attributes.

```html
<p style="font-family: &quot;Rotis Sans Cyrillic&quot;, sans-serif">test</p>
```
