---
"@biomejs/biome": patch
---

Fixed [#6893](https://github.com/biomejs/biome/issues/6893): The [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) rule now correctly adds a dependency that is captured in a shorthand object member. For example:

```jsx
useEffect(() => {
  console.log({firstId, secondId});
}, []);
```

is now correctly fixed to:

```jsx
useEffect(() => {
  console.log({firstId, secondId});
}, [firstId, secondId]);
```
