---
"@biomejs/biome": patch
---

Fixed [#8883](https://github.com/biomejs/biome/issues/8883): `useExhaustiveDependencies` no longer produces false positives when props are destructured in the function body of arrow function components without parentheses around the parameter.

```tsx
type Props = { msg: string };

// Arrow function without parentheses around `props`
const Component: React.FC<Props> = props => {
    const { msg } = props;
    // Previously, this incorrectly reported `msg` as unnecessary
    useEffect(() => console.log(msg), [msg]);
};
```
