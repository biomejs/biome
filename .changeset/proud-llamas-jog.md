---
"@biomejs/biome": minor
---

Added support for the typed `attr` function. Addresses issue #6183.

**Example**

``` css
.btn {
  width: attr(data-size type(<length> | <percentage>), 0px);
}
```
