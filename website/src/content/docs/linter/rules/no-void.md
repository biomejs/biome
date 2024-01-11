---
title: noVoid (since v1.0.0)
---

**Diagnostic Category: `lint/complexity/noVoid`**

Source: <a href="https://eslint.org/docs/latest/rules/no-void" target="_blank"><code>no-void</code></a>

Disallow the use of `void` operators, which is not a familiar operator.

>The `void` operator is often used merely to obtain the undefined primitive value,
usually using `void(0)` (which is equivalent to `void 0`). In these cases, the global variable `undefined` can be used.


## Examples

### Invalid

```jsx
void 0;
```

<pre class="language-text"><code class="language-text">complexity/noVoid.js:1:1 <a href="https://biomejs.dev/linter/rules/no-void">lint/complexity/noVoid</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The use of </span><span style="color: Orange;"><strong>void</strong></span><span style="color: Orange;"> is not allowed.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>void 0;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">If you use </span><span style="color: lightgreen;"><strong>void</strong></span><span style="color: lightgreen;"> to alter the return type of a function or return `undefined`, use the global `undefined` instead.</span>
  
</code></pre>

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
