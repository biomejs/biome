---
"@biomejs/biome": patch
---

Added the [`useConsistentBooleanProps`](https://biomejs.dev/linter/rules/use-consistent-boolean-props/) rule.
This rule enforces consistent usage of boolean props in JSX based on the configured mode (`implicit` or `explicit`).

**Invalid (implicit):**

```jsx
<input disabled={true} />;
```

**Valid (implicit):**

```jsx
<input disabled />;
```

**Invalid (explicit):**

```jsx
<input disabled />;
```

**Valid (explicit):**

```jsx
<input disabled={true} />;
```
