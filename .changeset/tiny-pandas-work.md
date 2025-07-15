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

**Updated:** The [`useJsxKeyInIterable`](https://biomejs.dev/linter/rules/use-jsx-key-in-iterable) rule now supports both React and Qwik domains. The previous Qwik-specific `noMissingJsxKey` rule has been merged into this unified rule. It disallows JSX elements in iterators/collections without a `key` prop, helping prevent rendering issues and performance problems in both React and Qwik applications.

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

Added the [`useJsxImg`](https://biomejs.dev/linter/rules/use-jsx-img) rule to Biome.

The `useJsxImg` rule enforces the use of width and height attributes on `<img>` elements for performance reasons. This rule is intended for use in Qwik applications to prevent layout shifts and improve Core Web Vitals by ensuring images have explicit dimensions.

**Invalid:**

```jsx
<img src="/image.png" />
<img src="https://example.com/image.png" />
<img src="/image.png" width="200" />
<img src="/image.png" height="200" />
```

**Valid:**

```jsx
<img width="200" height="600" src="/static/images/portrait-01.webp" />
<img width="100" height="100" src="https://example.com/image.png" />
```

The rule detects `<img>` elements that are missing either the `width` or `height` attribute and provides clear diagnostics to help developers add appropriate dimensions for optimal performance and user experience.

No configuration options are currently available for this rule.

Added the [`noUseVisibleTask`](https://biomejs.dev/linter/rules/no-use-visible-task) rule to Biome.

The `noUseVisibleTask` rule disallows the use of `useVisibleTask$()` functions in Qwik applications. This rule is intended for use in Qwik applications to prevent the use of `useVisibleTask$()` functions which are not recommended in Qwik.

**Invalid:**

```js
useVisibleTask$(() => {
  console.log('Component is visible');
});
```

**Valid:**

```js
useTask$(() => {
  console.log('Task executed');
});
```

The rule detects calls to `useVisibleTask$()` and provides clear diagnostics to help developers use alternative Qwik lifecycle functions like `useTask$()` instead.

No configuration options are currently available for this rule.

Added the [`useJsxA`](https://biomejs.dev/linter/rules/use-jsx-a) rule to Biome.

The `useJsxA` rule enforces the presence of an `href` attribute on `<a>` elements in JSX. This rule is intended for use in Qwik applications to ensure that anchor elements are always valid and accessible.

**Invalid:**

```jsx
<a>Link</a>
```

```jsx
<a target="_blank">External</a>
```

**Valid:**

```jsx
<a href="/home">Home</a>
```

```jsx
<a href="https://example.com" target="_blank">External</a>
```

The rule detects `<a>` elements that are missing the `href` attribute and provides clear diagnostics to help developers ensure all anchor elements are valid links.

No configuration options are currently available for this rule.
