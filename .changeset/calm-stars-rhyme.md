---
"@biomejs/biome": patch
---

Added a new lint rule [`noReactForwardRef`](https://biomejs.dev/linter/rules/no-react-forward-ref/), which detects usages of `forwardRef` that is no longer needed and deprecated in React 19.

For example:

```jsx
export const Component = forwardRef(function Component(props, ref) {
  return <div ref={ref} />;
});
```

will be fixed to:

```jsx
export const Component = function Component({ ref, ...props }) {
  return <div ref={ref} />;
};
```

Note that the rule provides an unsafe fix, which may break the code. Don't forget to review the code after applying the fix.
