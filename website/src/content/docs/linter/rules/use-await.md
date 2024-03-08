---
title: useAwait (since v1.4.0)
---

**Diagnostic Category: `lint/suspicious/useAwait`**

Source: <a href="https://eslint.org/docs/latest/rules/require-await" target="_blank"><code>require-await</code></a>

Ensure `async` functions utilize `await`.

This rule reports `async` functions that lack an `await` expression. As `async`
functions return a promise, the use of `await` is often necessary to capture the
resolved value and handle the asynchronous operation appropriately. Without `await`,
the function operates synchronously and might not leverage the advantages of async
functions.

## Examples

### Invalid

```jsx
async function fetchData() {
// Missing `await` for the promise returned by `fetch`
  return fetch('/data');
}
```

<pre class="language-text"><code class="language-text">suspicious/useAwait.js:1:1 <a href="https://biomejs.dev/linter/rules/use-await">lint/suspicious/useAwait</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>async</strong></span><span style="color: Orange;"> function lacks an </span><span style="color: Orange;"><strong>await</strong></span><span style="color: Orange;"> expression.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>async function fetchData() {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>// Missing `await` for the promise returned by `fetch`
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  return fetch('/data');
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>}
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Remove this </span><span style="color: lightgreen;"><strong>async</strong></span><span style="color: lightgreen;"> modifier, or add an </span><span style="color: lightgreen;"><strong>await</strong></span><span style="color: lightgreen;"> expression in the function.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>async function fetchData() {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>// Missing `await` for the promise returned by `fetch`
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  return fetch('/data');
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>}
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;"><strong>Async</strong></span><span style="color: lightgreen;"> functions without </span><span style="color: lightgreen;"><strong>await</strong></span><span style="color: lightgreen;"> expressions may not need to be declared </span><span style="color: lightgreen;"><strong>async</strong></span><span style="color: lightgreen;">.</span>
  
</code></pre>

### Valid

```jsx
async function fetchData() {
  const response = await fetch('/data');
  const data = await response.json();
  return data;
}

// This rule does not warn about non-async functions
function processData() {
  return compute(data);
}

// Nor does it warn about empty `async` functions
async function noop() { }
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
