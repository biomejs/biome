---
title: noMisrefactoredShorthandAssign (since v1.3.0)
---

**Diagnostic Category: `lint/suspicious/noMisrefactoredShorthandAssign`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://rust-lang.github.io/rust-clippy/master/#/misrefactored_assign_op" target="_blank"><code>misrefactored_assign_op</code></a>

Disallow shorthand assign when variable appears on both sides.

This rule helps to avoid potential bugs related to incorrect assignments or unintended
side effects that may occur during refactoring.

## Examples

### Invalid

```jsx
a += a + b
```

<pre class="language-text"><code class="language-text">suspicious/noMisrefactoredShorthandAssign.js:1:1 <a href="https://biomejs.dev/linter/rules/no-misrefactored-shorthand-assign">lint/suspicious/noMisrefactoredShorthandAssign</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Variable appears on both sides of an assignment operation.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>a += a + b
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This assignment might be the result of a wrong refactoring.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>a += b</strong></span><span style="color: lightgreen;"> instead.</span>
  
<strong>  </strong><strong>  1 │ </strong>a<span style="opacity: 0.8;">·</span>+=<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">+</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>b
<strong>  </strong><strong>    │ </strong>     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span> 
</code></pre>

```jsx
a -= a - b
```

<pre class="language-text"><code class="language-text">suspicious/noMisrefactoredShorthandAssign.js:1:1 <a href="https://biomejs.dev/linter/rules/no-misrefactored-shorthand-assign">lint/suspicious/noMisrefactoredShorthandAssign</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Variable appears on both sides of an assignment operation.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>a -= a - b
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This assignment might be the result of a wrong refactoring.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>a -= b</strong></span><span style="color: lightgreen;"> instead.</span>
  
<strong>  </strong><strong>  1 │ </strong>a<span style="opacity: 0.8;">·</span>-=<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">-</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>b
<strong>  </strong><strong>    │ </strong>     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span> 
</code></pre>

```jsx
a *= a * b
```

<pre class="language-text"><code class="language-text">suspicious/noMisrefactoredShorthandAssign.js:1:1 <a href="https://biomejs.dev/linter/rules/no-misrefactored-shorthand-assign">lint/suspicious/noMisrefactoredShorthandAssign</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Variable appears on both sides of an assignment operation.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>a *= a * b
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This assignment might be the result of a wrong refactoring.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>a *= b</strong></span><span style="color: lightgreen;"> instead.</span>
  
<strong>  </strong><strong>  1 │ </strong>a<span style="opacity: 0.8;">·</span>*=<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">*</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>b
<strong>  </strong><strong>    │ </strong>     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span> 
</code></pre>

### Valid

```jsx
a += b
```

```jsx
a = a + b
```

```jsx
a = a - b
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
