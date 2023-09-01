---
title: noVoid (since v1.0.0)
---


Disallow the use of `void` operators, which is not a familiar operator.

>The `void` operator is often used merely to obtain the undefined primitive value,
usually using `void(0)` (which is equivalent to `void 0`). In these cases, the global variable `undefined` can be used.


Source: https://eslint.org/docs/latest/rules/no-void

## Examples

### Invalid

```jsx
void 0;
```

<pre class="language-text"><code class="language-text">nursery/noVoid.js:1:1 <a href="https://biomejs.dev/linter/rules/no-void">lint/nursery/noVoid</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The use of </span><span style="color: Orange;"><strong>void</strong></span><span style="color: Orange;"> is not allowed.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>void 0;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">If you use </span><span style="color: rgb(38, 148, 255);"><strong>void</strong></span><span style="color: rgb(38, 148, 255);"> to alter the return type of a function or return `undefined`, use the global `undefined` instead.</span>
  
</code></pre>

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
