---
title: noApproximativeNumericConstant (since v1.3.0)
---

**Diagnostic Category: `lint/suspicious/noApproximativeNumericConstant`**

Usually, the definition in the standard library is more precise than what people come up with or the used constant exceeds the maximum precision of the number type.

Source: https://rust-lang.github.io/rust-clippy/master/#approx_constant

## Examples

### Invalid

```jsx
let x = 3.141;
```

<pre class="language-text"><code class="language-text">suspicious/noApproximativeNumericConstant.js:1:9 <a href="https://biomejs.dev/linter/rules/no-approximative-numeric-constant">lint/suspicious/noApproximativeNumericConstant</a> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Prefer constants from the standard library.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let x = 3.141;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>Math.PI</strong></span><span style="color: lightgreen;"> instead.</span>
  
</code></pre>

```jsx
let x = 2.302;
```

<pre class="language-text"><code class="language-text">suspicious/noApproximativeNumericConstant.js:1:9 <a href="https://biomejs.dev/linter/rules/no-approximative-numeric-constant">lint/suspicious/noApproximativeNumericConstant</a> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Prefer constants from the standard library.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let x = 2.302;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>Math.LN10</strong></span><span style="color: lightgreen;"> instead.</span>
  
</code></pre>

## Valid

```jsx
let x = Math.PI;
```

```jsx
let x = Math.LN10;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
