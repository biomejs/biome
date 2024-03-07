---
title: noUndeclaredDependencies (since v1.6.0)
---

**Diagnostic Category: `lint/nursery/noUndeclaredDependencies`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow the use of dependencies that aren't specified in the `package.json`.

Indirect dependencies will trigger the rule because they aren't declared in the `package.json`. This means that if package `@org/foo` has a dependency on `lodash`, and then you use
`import "lodash"` somewhere in your project, the rule will trigger a diagnostic for this import.

The rule ignores imports using a protocol such as `node:`, `bun:`, `jsr:`, `https:`.

## Examples

### Invalid

```jsx
import "vite";
```

### Valid

```jsx
import { A } from "./local.js";
```

```jsx
import assert from "node:assert";
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
