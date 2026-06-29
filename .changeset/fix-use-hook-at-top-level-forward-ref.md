---
"@biomejs/biome": patch
---

Fixed [#9195](https://github.com/biomejs/biome/issues/9195): [`useHookAtTopLevel`](https://biomejs.dev/linter/rules/use-hook-at-top-level/) no longer reports a false positive for hooks called at the top level of a `forwardRef` component written as a two-parameter function declaration whose second parameter is `ref`.

```jsx
// This component is passed to `forwardRef` elsewhere, so it takes a second
// `ref` parameter. The hook is at the top level of the component and is no
// longer flagged.
function MyComponent(props, ref) {
	useEffect(() => {}, []);
	return <div ref={ref} />;
}
const Forwarded = forwardRef(MyComponent);
```
