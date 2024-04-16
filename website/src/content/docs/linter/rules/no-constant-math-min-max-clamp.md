---
title: noConstantMathMinMaxClamp (since v1.7.0)
---

**Diagnostic Category: `lint/nursery/noConstantMathMinMaxClamp`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Source: <a href="https://rust-lang.github.io/rust-clippy/master/#/min_max" target="_blank"><code>min_max</code></a>

Disallow the use of `Math.min` and `Math.max` to clamp a value where the result itself is constant.

## Examples

### Invalid

```jsx
Math.min(0, Math.max(100, x));
```

<pre class="language-text"><code class="language-text">nursery/noConstantMathMinMaxClamp.jsx:1:1 <a href="https://biomejs.dev/linter/rules/no-constant-math-min-max-clamp">lint/nursery/noConstantMathMinMaxClamp</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>Math.min/Math.max</strong></span><span style="color: Orange;"> combination leads to a constant result.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>Math.min(0, Math.max(100, x));
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">It always evaluates to </span><span style="color: lightgreen;"><strong>0</strong></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>Math.min(0, Math.max(100, x));
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Swap </span><span style="color: lightgreen;"><strong>0</strong></span><span style="color: lightgreen;"> with </span><span style="color: lightgreen;"><strong>100</strong></span><span style="color: lightgreen;">.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">M</span><span style="color: Tomato;">a</span><span style="color: Tomato;">t</span><span style="color: Tomato;">h</span><span style="color: Tomato;">.</span><span style="color: Tomato;">m</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">(</span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;">,</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">M</span><span style="color: Tomato;">a</span><span style="color: Tomato;">t</span><span style="color: Tomato;">h</span><span style="color: Tomato;">.</span><span style="color: Tomato;">m</span><span style="color: Tomato;">a</span><span style="color: Tomato;">x</span><span style="color: Tomato;">(</span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;">,</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">x</span><span style="color: Tomato;">)</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">M</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">h</span><span style="color: MediumSeaGreen;">.</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;">,</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">M</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">h</span><span style="color: MediumSeaGreen;">.</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;">,</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
Math.max(100, Math.min(0, x));
```

<pre class="language-text"><code class="language-text">nursery/noConstantMathMinMaxClamp.jsx:1:1 <a href="https://biomejs.dev/linter/rules/no-constant-math-min-max-clamp">lint/nursery/noConstantMathMinMaxClamp</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>Math.min/Math.max</strong></span><span style="color: Orange;"> combination leads to a constant result.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>Math.max(100, Math.min(0, x));
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">It always evaluates to </span><span style="color: lightgreen;"><strong>100</strong></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>Math.max(100, Math.min(0, x));
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Swap </span><span style="color: lightgreen;"><strong>100</strong></span><span style="color: lightgreen;"> with </span><span style="color: lightgreen;"><strong>0</strong></span><span style="color: lightgreen;">.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">M</span><span style="color: Tomato;">a</span><span style="color: Tomato;">t</span><span style="color: Tomato;">h</span><span style="color: Tomato;">.</span><span style="color: Tomato;">m</span><span style="color: Tomato;">a</span><span style="color: Tomato;">x</span><span style="color: Tomato;">(</span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;">,</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">M</span><span style="color: Tomato;">a</span><span style="color: Tomato;">t</span><span style="color: Tomato;">h</span><span style="color: Tomato;">.</span><span style="color: Tomato;">m</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">(</span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;">,</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">x</span><span style="color: Tomato;">)</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">M</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">h</span><span style="color: MediumSeaGreen;">.</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;">,</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">M</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">h</span><span style="color: MediumSeaGreen;">.</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;">,</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
Math.min(100, Math.max(0, x));
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
