---
"@biomejs/biome": patch
---

Fixed `{` inside an Astro `<math>` element opening an expression. MathML is foreign content where Astro parses no expressions, so LaTeX such as `R^{2x}` now survives as text. `<svg>` is unaffected.

```astro
<math><annotation>R^{2x}</annotation></math>
```
