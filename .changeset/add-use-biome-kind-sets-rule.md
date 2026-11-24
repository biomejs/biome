---
"@biomejs/biome": patch
---

Added the nursery rule [`useBiomeKindSets`](https://biomejs.dev/linter/rules/use-biome-kind-sets/), which reports Biome JS plugins that check the `kind` of a node or token against several strings. A kind set created with `nodeKinds()` or `tokenKinds()` reads the kind natively, which is faster.

The rule belongs to the new `biomePlugin` domain. Biome doesn't enable this domain automatically, so enable it in the linter configuration of your plugin project.

```js
if (node.kind === "JS_CALL_EXPRESSION" || node.kind === "JS_NEW_EXPRESSION") {}
```
