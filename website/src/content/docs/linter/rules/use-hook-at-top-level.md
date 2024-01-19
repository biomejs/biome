---
title: useHookAtTopLevel (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/useHookAtTopLevel`**

Source: <a href="https://github.com/facebook/react/blob/main/packages/eslint-plugin-react-hooks/README.md" target="_blank"><code>rules-of-hooks</code></a>

Enforce that all React hooks are being called from the Top Level component functions.

To understand why this required see https://reactjs.org/docs/hooks-rules.html#only-call-hooks-at-the-top-level

## Examples

### Invalid

```jsx
function Component1({ a }) {
    if (a == 1) {
        useEffect();
    }
}
```

<pre class="language-text"><code class="language-text">correctness/useHookAtTopLevel.js:3:9 <a href="https://biomejs.dev/linter/rules/use-hook-at-top-level">lint/correctness/useHookAtTopLevel</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This hook is being called conditionally, but all hooks must be called in the exact same order in every component render.</span>
  
    <strong>1 │ </strong>function Component1({ a }) {
    <strong>2 │ </strong>    if (a == 1) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        useEffect();
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>    }
    <strong>5 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">For React to preserve state between calls, hooks needs to be called unconditionally and always in the same order.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">See https://reactjs.org/docs/hooks-rules.html#only-call-hooks-at-the-top-level</span>
  
</code></pre>

```jsx
function Component1({ a }) {
    if (a != 1) {
        return;
    }

    useEffect();
}
```

<pre class="language-text"><code class="language-text">correctness/useHookAtTopLevel.js:6:5 <a href="https://biomejs.dev/linter/rules/use-hook-at-top-level">lint/correctness/useHookAtTopLevel</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This hook is being called conditionally, but all hooks must be called in the exact same order in every component render.</span>
  
    <strong>4 │ </strong>    }
    <strong>5 │ </strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>    useEffect();
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>7 │ </strong>}
    <strong>8 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Hooks should not be called after an early return.</span>
  
    <strong>1 │ </strong>function Component1({ a }) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    if (a != 1) {
   <strong>   │ </strong>                 
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        return;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>    }
    <strong>5 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">For React to preserve state between calls, hooks needs to be called unconditionally and always in the same order.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">See https://reactjs.org/docs/hooks-rules.html#only-call-hooks-at-the-top-level</span>
  
</code></pre>

### Valid

```jsx
function Component1() {
    useEffect();
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
