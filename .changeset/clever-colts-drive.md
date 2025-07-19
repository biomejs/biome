---
"@biomejs/backend-jsonrpc": patch
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

The rule detects calls to `useVisibleTask$()` and provides clear diagnostics to help developers use alternative Qwik lifecycle functions like `useTask$()` instead.