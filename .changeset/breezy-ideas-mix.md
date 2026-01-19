---
"@biomejs/biome": patch
---

Properly handle `parameters` metavariables for `arrow_function` GritQL queries. The following `biome search` command no longer throws an error:

```shell
biome search 'arrow_function(parameters=$parameters, body=$body)'
```