---
title: noMisrefactoredShorthandAssign (since vnext)
---

**Diagnostic Category: `lint/nursery/noMisrefactoredShorthandAssign`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow shorthand assign when variable appears on both sides.

This rule helps avoid potential bugs related to incorrect assignments or unintended
side effects that may occur during refactoring.

Source: https://rust-lang.github.io/rust-clippy/master/#/misrefactored_assign_op

## Examples

### Invalid

```jsx
a += a + b
```

<pre class="language-text"><code class="language-text">nursery/noMisrefactoredShorthandAssign.js:1:1 <a href="https://biomejs.dev/lint/rules/no-misrefactored-shorthand-assign">lint/nursery/noMisrefactoredShorthandAssign</a> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Variable appears on both sides of an assignment operation</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>a += a + b
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
a -= a - b
```

<pre class="language-text"><code class="language-text">nursery/noMisrefactoredShorthandAssign.js:1:1 <a href="https://biomejs.dev/lint/rules/no-misrefactored-shorthand-assign">lint/nursery/noMisrefactoredShorthandAssign</a> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Variable appears on both sides of an assignment operation</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>a -= a - b
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
a *= a * b
```

<pre class="language-text"><code class="language-text">nursery/noMisrefactoredShorthandAssign.js:1:1 <a href="https://biomejs.dev/lint/rules/no-misrefactored-shorthand-assign">lint/nursery/noMisrefactoredShorthandAssign</a> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Variable appears on both sides of an assignment operation</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>a *= a * b
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

## Valid

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
