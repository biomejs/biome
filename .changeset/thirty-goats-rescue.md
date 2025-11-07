---
"@biomejs/biome": minor
---

Added the `allowedModules` configuration option to the [`noNamespaceImport`](https://biomejs.dev/linter/rules/no-namespace-import/) rule.

This option allows you to specify module specifiers that are permitted to use namespace imports. Both external dependencies and local modules (relative/absolute paths) are supported. This is useful for:
- Libraries designed to work with namespace imports (such as Zod or Valibot)
- Local modules that export many utilities

**Example configuration:**

```json
{
  "linter": {
    "rules": {
      "performance": {
        "noNamespaceImport": {
          "level": "error",
          "options": {
            "allowedModules": ["zod", "./utils/helpers"]
          }
        }
      }
    }
  }
}
```

With this configuration, namespace imports from the specified modules will not trigger the rule:

```js
import * as z from "zod";              // No error - external dependency in allowlist
import * as helpers from "./utils/helpers";  // No error - local module in allowlist
import * as foo from "foo";            // Error - not in allowed modules
import * as other from "./other";      // Error - not in allowed modules
```