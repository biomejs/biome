---
"@biomejs/biome": minor
---

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

The pre-existing `useJsxKeyInIterable` rule is now available in the Qwik domain.

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

Added the [`noQwikUseVisibleTask`](https://biomejs.dev/linter/rules/no-qwik-use-visible-task) rule to Biome.

This rule is intended for use in Qwik applications to warn about the use of `useVisibleTask$()` functions which require careful consideration before use.

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

Added the [`useAnchorHref`](https://biomejs.dev/linter/rules/use-anchor-href) rule to Biome.

The `useAnchorHref` rule enforces the presence of an `href` attribute on `<a>` elements in JSX. This rule is intended for use in Qwik applications to ensure that anchor elements are always valid and accessible.

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
