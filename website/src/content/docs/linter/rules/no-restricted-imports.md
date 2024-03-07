---
title: noRestrictedImports (since v1.6.0)
---

**Diagnostic Category: `lint/nursery/noRestrictedImports`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-restricted-imports" target="_blank"><code>no-restricted-imports</code></a>

Disallow specified modules when loaded by import or require.

## Options

```json
{
    "noRestrictedImports": {
        "options": {
            "paths": {
                "lodash": "Using lodash is not encouraged",
                "underscore": "Using underscore is not encouraged"
            }
        }
    }
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
