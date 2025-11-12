---
"@biomejs/biome": patch
---

Fixed [#8080](https://github.com/biomejs/biome/issues/8080): The HTML parser, when parsing Vue, can now properly handle Vue directives with no argument, modifiers, or initializer (eg. `v-else`). It will no longer treat subsequent valid attributes as bogus.

```vue
<p v-else class="flex">World</p> <!-- Fixed: class now gets parsed as it's own attribute -->
```
