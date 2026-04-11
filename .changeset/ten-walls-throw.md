---
"@biomejs/biome": patch
---

Added the nursery rule [`noJsxLeakedSemicolon`](https://biomejs.dev/linter/rules/no-jsx-leaked-semicolon), which disallows leaked semicolons in JSX text nodes.

**Invalid**:

```jsx
const MyComponent = () => {
	return (
		<div>
			<div />;
		</div>
	);
}
```
