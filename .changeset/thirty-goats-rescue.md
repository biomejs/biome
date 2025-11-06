---
"@biomejs/biome": minor
---

Added the `allowlist` configuration option to the [`noNamespaceImport`](https://biomejs.dev/linter/rules/no-namespace-import/) rule.

This option allows you to specify modules that are permitted to use namespace imports, which is useful for libraries designed to work with namespace imports (such as Zod or Valibot) or when you need to import many exports from a module.

**Example configuration:**

```json
{
  "linter": {
    "rules": {
      "performance": {
        "noNamespaceImport": {
          "level": "error",
          "options": {
            "allowlist": ["zod", "valibot"]
          }
        }
      }
    }
  }
}
```

With this configuration, namespace imports from the specified modules will not trigger the rule:

```js
import * as z from "zod";      // ✅ No error
import * as v from "valibot";  // ✅ No error
import * as foo from "foo";    // ❌ Error - not in allowlist
```