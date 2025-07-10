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

Added the [`noMissingJsxKey`](https://biomejs.dev/linter/rules/no-missing-jsx-key) rule to Biome.

The `noMissingJsxKey` rule disallows JSX elements in iterators/collections without a `key` prop. This rule is intended for use in Qwik applications to prevent missing key props in JSX elements inside iterators, which can lead to rendering issues and performance problems.

**Invalid:**

```jsx
{items.map(item => <li>{item}</li>)}
{users.map(user => <div>{user.name}</div>)}
```

**Valid:**

```jsx
{items.map((item, index) => <li key={index}>{item}</li>)}
{users.map(user => <div key={user.id}>{user.name}</div>)}
```

The rule detects JSX elements inside `.map()` calls and other iterator methods that are missing the required `key` prop. It provides clear diagnostics to help developers add appropriate keys for optimal rendering performance.

No configuration options are currently available for this rule.
