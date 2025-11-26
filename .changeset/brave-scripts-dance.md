---
"@biomejs/biome": patch
---

Added the new nursery rule [`useRequiredScripts`](https://biomejs.dev/linter/rules/use-required-scripts/), which enforces the presence of required scripts in `package.json`. This rule is particularly useful in monorepo environments where consistency across workspaces is important.

The rule accepts a `requiredScripts` option to specify which scripts must be present:

```json
{
    "linter": {
        "rules": {
            "nursery": {
                "useRequiredScripts": {
                    "level": "error",
                    "options": {
                        "requiredScripts": ["test", "build", "lint"]
                    }
                }
            }
        }
    }
}
```

For example, the following `package.json` triggers the rule when `["test", "build"]` are required:

```json
{
    "scripts": {
        "test": "vitest"
    }
}
```

```
package.json:1:1 lint/nursery/useRequiredScripts ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ The required script "build" is missing from package.json.

  > 1 │ {
      │ ^
    2 │     "scripts": {
    3 │         "test": "vitest"

  ℹ Add the missing script to your package.json.
```
