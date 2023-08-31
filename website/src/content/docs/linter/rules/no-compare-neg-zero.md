---
title: noCompareNegZero (since v1.0.0)
---


:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Disallow comparing against `-0`

## Examples

### Invalid

```jsx
(1 >= -0)
```

<pre class="language-text"><code class="language-text">suspicious/noCompareNegZero.js:1:2 <a href="https://biomejs.dev/lint/rules/noCompareNegZero">lint/suspicious/noCompareNegZero</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not use the &gt;= operator to compare against -0.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>(1 &gt;= -0)
   <strong>   │ </strong> <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace -0 with 0</span>
  
<strong>  </strong><strong>  1 │ </strong>(1<span style="opacity: 0.8;">·</span>&gt;=<span style="opacity: 0.8;">·</span><span style="color: Tomato;">-</span>0)
<strong>  </strong><strong>    │ </strong>      <span style="color: Tomato;">-</span>  
</code></pre>

### Valid

```jsx
(1 >= 0)
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
