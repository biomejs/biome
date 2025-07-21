---
"@biomejs/biome": patch
---

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
