---
title: noUndeclaredVariables (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/noUndeclaredVariables`**

Source: <a href="https://eslint.org/docs/latest/rules/no-undef" target="_blank"><code>no-undef</code></a>

Prevents the usage of variables that haven't been declared inside the document.

If you need to allow-list some global bindings, you can use the [`javascript.globals`](/reference/configuration/#javascriptglobals) configuration.

## Examples

### Invalid

```jsx
foobar;
```

<pre class="language-text"><code class="language-text">correctness/noUndeclaredVariables.js:1:1 <a href="https://biomejs.dev/linter/rules/no-undeclared-variables">lint/correctness/noUndeclaredVariables</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The </span><span style="color: Orange;"><strong>foobar</strong></span><span style="color: Orange;"> variable is undeclared</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>foobar;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
// throw diagnostic for JavaScript files
PromiseLike;
```

<pre class="language-text"><code class="language-text">correctness/noUndeclaredVariables.js:2:1 <a href="https://biomejs.dev/linter/rules/no-undeclared-variables">lint/correctness/noUndeclaredVariables</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The </span><span style="color: Orange;"><strong>PromiseLike</strong></span><span style="color: Orange;"> variable is undeclared</span>
  
    <strong>1 │ </strong>// throw diagnostic for JavaScript files
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>PromiseLike;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
</code></pre>

### Valid

```ts
type B<T> = PromiseLike<T>
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
