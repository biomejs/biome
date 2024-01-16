---
title: noUnsafeNegation (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noUnsafeNegation`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-unsafe-negation" target="_blank"><code>no-unsafe-negation</code></a>

Disallow using unsafe negation.

## Examples

### Invalid

```jsx
!1 in [1,2];
```

<pre class="language-text"><code class="language-text">suspicious/noUnsafeNegation.js:1:1 <a href="https://biomejs.dev/linter/rules/no-unsafe-negation">lint/suspicious/noUnsafeNegation</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The negation operator is used unsafely on the left side of this binary expression.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>!1 in [1,2];
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Wrap the expression with a parenthesis</span>
  
<strong>  </strong><strong>  1 │ </strong>!<span style="color: MediumSeaGreen;">(</span>1<span style="opacity: 0.8;">·</span>in<span style="opacity: 0.8;">·</span>[1,2]<span style="color: MediumSeaGreen;">)</span>;
<strong>  </strong><strong>    │ </strong> <span style="color: MediumSeaGreen;">+</span>          <span style="color: MediumSeaGreen;">+</span> 
</code></pre>

```jsx
/**test*/!/** test*/1 instanceof [1,2];
```

<pre class="language-text"><code class="language-text">suspicious/noUnsafeNegation.js:1:10 <a href="https://biomejs.dev/linter/rules/no-unsafe-negation">lint/suspicious/noUnsafeNegation</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The negation operator is used unsafely on the left side of this binary expression.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>/**test*/!/** test*/1 instanceof [1,2];
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Wrap the expression with a parenthesis</span>
  
<strong>  </strong><strong>  1 │ </strong>/**test*/!/**<span style="opacity: 0.8;">·</span>test*/<span style="color: MediumSeaGreen;">(</span>1<span style="opacity: 0.8;">·</span>instanceof<span style="opacity: 0.8;">·</span>[1,2]<span style="color: MediumSeaGreen;">)</span>;
<strong>  </strong><strong>    │ </strong>                    <span style="color: MediumSeaGreen;">+</span>                  <span style="color: MediumSeaGreen;">+</span> 
</code></pre>

### Valid

```jsx
-1 in [1,2];
~1 in [1,2];
typeof 1 in [1,2];
void 1 in [1,2];
delete 1 in [1,2];
+1 instanceof [1,2];
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
