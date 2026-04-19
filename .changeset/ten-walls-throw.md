---
"@biomejs/biome": patch
---

Added the nursery rule [`noJsxLeakedSemicolon`](https://biomejs.dev/linter/rules/no-jsx-leaked-semicolon), which flags text nodes with a leading `;` after a JSX element.
This could be an unintentional mistake, resulting in a ';' being rendered as text in the output.

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
