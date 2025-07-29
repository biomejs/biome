---
"@biomejs/biome": patch
---


Added the [`noQwikUseVisibleTask`](https://biomejs.dev/linter/rules/no-qwik-use-visible-task) rule to Qwik.

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

