---
"@biomejs/biome": patch
---
[noUselessFragments](https://biomejs.dev/linter/rules/no-useless-fragments/) now handles `JsxAttributeInitializerClause`, ensuring that fragments inside expressions like `<A b=<></> />` are preserved. ([#4208](https://github.com/biomejs/biome/issues/4208)).
