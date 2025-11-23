---
"@biomejs/biome": patch
---

Fixed [#8079](https://github.com/biomejs/biome/issues/8079): Properly handle `name` and `value` metavariables for `JsxAttribute` GritQL queries.

The following `biome search` command no longer throws an error:

```
biome search 'JsxAttribute($name, $value) as $attr where { $name <: "style" }'
```
