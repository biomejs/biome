---
"@biomejs/biome": patch
---

Fixed [#9140](https://github.com/biomejs/biome/issues/9140): Biome now parses Astro's attribute shorthand inside `.astro` files. The following snippet no longer reports a parse error:

```astro
---
const items = ['a', 'b'];
---
<ul>
  {items.map((item) => <li {item}>row</li>)}
</ul>
```
