---
"@biomejs/biome": patch
---

Fixed [#10636](https://github.com/biomejs/biome/issues/10636): [noStaticElementInteractions](https://biomejs.dev/linter/rules/no-static-element-interactions/) no longer reports a false positive for event handlers on Svelte special elements such as `<svelte:window>`, `<svelte:document>`, and `<svelte:body>`. These are not real DOM elements, so they are now ignored by the rule.
