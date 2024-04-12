---
title: noReactSpecificProps (since v1.0.0)
---

**Diagnostic Category: `lint/nursery/noReactSpecificProps`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Source: <a href="https://github.com/solidjs-community/eslint-plugin-solid/blob/main/docs/no-react-specific-props.md" target="_blank"><code>no-react-specific-props</code></a>

Prevents React-specific JSX properties from being used. This may be desirable when using SolidJS.

## Examples

### Invalid

```jsx
<Hello className="text-red" />
```

### Valid

```jsx
<Hello class="text-red" />
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
