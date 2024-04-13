---
title: componentsReturnOnce (since v1.0.0)
---

**Diagnostic Category: `lint/nursery/componentsReturnOnce`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Source: <a href="https://github.com/solidjs-community/eslint-plugin-solid/blob/main/docs/components-return-once.md" target="_blank"><code>components-return-once</code></a>

Disallow early returns in components. Solid components only run once, and so conditionals should be inside JSX.

## Examples

### Invalid

```jsx
function Component() {
  if (condition) {
    return <div />;
  }
  return <span />;
}
```

### Valid

```jsx
function Component() {
  return <div />;
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)