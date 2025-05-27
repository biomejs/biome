---
"@biomejs/biome": patch
---

Fixed [#4715](https://github.com/biomejs/biome/issues/4715): The `useJsxKeyInIterable` rule now reports missing keys inside `switch` and `if` statements.

```jsx
const data = [
	{ value: 'a', type: 'string' },
	{ value: 9, type: 'number' },
	{ value: 'c', type: 'string' },
];

const MyComponent = () => {
	return (
		<>
      {/* if statements */}
			{data.map((x) => {
				if (x.type === 'string') {
					return <div>{x.value}</div> // no key, emits diagnostic
				} else {
					return <div>{x.value}</div> // no key, emits diagnostic
				}
			})}

      {/* switch statements */}
			{data.map((x) => {
				switch (x.type) {
					case 'string':
						return <div>{x.value}</div>; // no key, emits diagnostic
					case 'number':
						return <div>{x.value}</div>; // no key, emits diagnostic
					default:
						return <div key={x.value}>{x.value}</div>;
				}
			})}
		</>
	);
};
```
