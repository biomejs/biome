---
title: noDoneCallback (not released)
---

**Diagnostic Category: `lint/nursery/noDoneCallback`**

:::danger
This rule hasn't been released yet.
:::

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Source: <a href="https://github.com/jest-community/eslint-plugin-jest/blob/main/docs/rules/no-done-callback.md" target="_blank"><code>no-done-callback</code></a>

Disallow using a callback in asynchronous tests and hooks.

This rule checks the function parameter of hooks & tests for use of the done argument, suggesting you return a promise instead.

## Examples

### Invalid

```jsx
beforeEach(done => {
    // ...
});
```

<pre class="language-text"><code class="language-text">nursery/noDoneCallback.js:1:12 <a href="https://biomejs.dev/linter/rules/no-done-callback">lint/nursery/noDoneCallback</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Disallow using a callback in asynchronous tests and hooks.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>beforeEach(done =&gt; {
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    // ...
    <strong>3 │ </strong>});
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Return a Promise instead of relying on callback parameter.</span>
  
</code></pre>

```jsx
test('myFunction()', done => {
    // ...
});
```

<pre class="language-text"><code class="language-text">nursery/noDoneCallback.js:1:22 <a href="https://biomejs.dev/linter/rules/no-done-callback">lint/nursery/noDoneCallback</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Disallow using a callback in asynchronous tests and hooks.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>test('myFunction()', done =&gt; {
   <strong>   │ </strong>                     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    // ...
    <strong>3 │ </strong>});
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Return a Promise instead of relying on callback parameter.</span>
  
</code></pre>

### Valid

```jsx
beforeEach(async () => {
    // ...
});
```

```jsx
test('myFunction()', () => {
    expect(myFunction()).toBeTruthy();
});
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
