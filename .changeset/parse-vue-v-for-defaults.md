---
"@biomejs/biome": patch
---

Fixed parsing of default values in Vue `v-for` bindings. Biome now parses the default value as JavaScript, so it is formatted and linted like other template expressions instead of being reported as a parse error.

```vue
<div v-for="{ value = gt('v-for default') } in rows" />
```
