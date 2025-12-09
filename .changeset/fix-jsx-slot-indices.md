---
"@biomejs/biome": patch
---

Properly handle `name`, `type_arguments`, and `attributes` slots for `JsxOpeningElement` and `JsxSelfClosingElement` GritQL patterns.

The following biome search commands no longer throw errors:

```shell
biome search 'JsxOpeningElement(name = $elem_name) where { $elem_name <: "div" }'
biome search 'JsxSelfClosingElement(name = $elem_name) where { $elem_name <: "div" }'
```
