---
"@biomejs/biome": minor
---

Added the new rule [`useJsonImportAttribute`](https://biomejs.dev/linter/rules/use-json-import-attribute) to enforce the use of import attributes for JSON modules.

This rule ensures that all imports of `.json` files include the `with { type: "json" }` assertion, which is required to inform the JavaScript runtime that the imported file should be parsed as JSON.

```diff
- import jsonData from './data.json';
+ import jsonData from './data.json' with { type: "json" };
```

```diff
- import jsonData from './data.json' with { someOtherAttribute: "value" };
+ import jsonData from './data.json' with { type: "json", someOtherAttribute: "value" };
```

This rule is based on the proposal in issue [#6043](https://github.com/biomejs/biome/issues/6043).
