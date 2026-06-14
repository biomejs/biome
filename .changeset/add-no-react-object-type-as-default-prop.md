---
"@biomejs/biome": patch
---

Added [`noReactObjectTypeAsDefaultProp`](https://biomejs.dev/linter/rules/no-react-object-type-as-default-prop/) to the nursery group.

This rule reports arrays, objects, functions, and other non-primitive values used as default values for destructured props in React function components. Such a value is created again on every render, so React treats the prop as changed and may re-render the component more than needed. It ports [`no-object-type-as-default-prop`](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/no-object-type-as-default-prop.md) from `eslint-plugin-react`.
