---
"@biomejs/biome": patch
---

Added the [`useQwikClasslist`](https://biomejs.dev/linter/rules/use-qwik-classlist) rule to Biome.

This rule is intended for use in Qwik applications to encourage the use of the built-in `class` prop (which accepts a string, object, or array) instead of the `classnames` utility library.

**Invalid:**

```jsx
<div class={classnames({ active: true, disabled: false })} />
```

**Valid:**

```jsx
<div classlist={{ active: true, disabled: false }} />
```
