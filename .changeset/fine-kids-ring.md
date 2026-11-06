---
"@biomejs/biome": patch
---

Fixed [#5242](https://github.com/biomejs/biome/issues/5242): the HTML-family formatter now collapses whitespace in static `class` attributes and formats valid `style` attributes as inline CSS declarations. Component props, dynamic Astro template values, and style attributes containing invalid CSS remain conservative. Embedded replacements also recompute parent layout so formatting converges in one pass.

```diff
-<div class="  card   selected  " style="color:red;font-size:10px"></div>
+<div class="card selected" style="color: red; font-size: 10px;"></div>
```
