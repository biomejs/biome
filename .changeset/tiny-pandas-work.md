---
"@biomejs/biome": minor
---

Added Qwik domain support and the [`noReactProps`](https://biomejs.dev/linter/rules/no-react-props) rule to Biome.

The Qwik domain provides framework-specific linting rules for Qwik applications. Biome now automatically enables Qwik rules when it detects Qwik dependencies in the project.

The `noReactProps` rule disallows React-specific JSX attributes (`className`, `htmlFor`) in Qwik components and suggests using standard HTML attributes instead.

**Invalid:**

```jsx
<div className="container" />
<label htmlFor="input" />
```

**Valid:**

```jsx
<div class="container" />
<label for="input" />
```

The rule provides automatic fixes to replace React-specific props with their HTML equivalents.
