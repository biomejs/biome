---
title: noUndeclaredDependencies (not released)
---

**Diagnostic Category: `lint/nursery/noUndeclaredDependencies`**

:::danger
This rule hasn't been released yet.
:::

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow the use of dependencies that aren't specified in the `package.json`.

Indirect dependencies will trigger the rule because they aren't declared in the `package.json`. This means that if package `@org/foo` has a dependency on `lodash`, and then you use
`import "lodash"` somewhere in your project, the rule will trigger a diagnostic for this import.

## Examples

### Invalid

```jsx
import "vite";
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
