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

Added the [`preferClasslist`](https://biomejs.dev/linter/rules/prefer-classlist) rule to Biome.

The `preferClasslist` rule encourages the use of the Qwik-specific `classlist` prop instead of the `classnames` utility in JSX. This helps Qwik applications leverage the built-in prop for conditional class application, improving compatibility and code clarity.

**Invalid:**

```jsx
<div class={classnames({ active: true, disabled: false })} />
```

**Valid:**

```jsx
<div classlist={{ active: true, disabled: false }} />
```

The rule detects usage of the `classnames` helper in `class` or `className` attributes and recommends switching to the `classlist` prop, which accepts an object mapping class names to booleans.

No configuration options are currently available for this rule.
