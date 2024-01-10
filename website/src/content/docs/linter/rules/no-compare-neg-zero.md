---
title: noCompareNegZero (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noCompareNegZero`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-compare-neg-zero" target="_blank"><code>no-compare-neg-zero</code></a>

Disallow comparing against `-0`

## Examples

### Invalid

```jsx
(1 >= -0)
```

<pre class="language-text"><code class="language-text">suspicious/noCompareNegZero.js:1:2 <a href="https://biomejs.dev/linter/rules/no-compare-neg-zero">lint/suspicious/noCompareNegZero</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not use the &gt;= operator to compare against -0.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>(1 &gt;= -0)
   <strong>   │ </strong> <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Replace -0 with 0</span>
  
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
