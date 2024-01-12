---
title: useValidAriaProps (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/useValidAriaProps`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-props.md" target="_blank"><code>aria-props</code></a>

Ensures that ARIA properties `aria-*` are all valid.

## Examples

### Invalid

```jsx
<input className="" aria-labell="" />
```

<pre class="language-text"><code class="language-text">a11y/useValidAriaProps.js:1:1 <a href="https://biomejs.dev/linter/rules/use-valid-aria-props">lint/a11y/useValidAriaProps</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The element contains invalid ARIA attribute(s)</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;input className=&quot;&quot; aria-labell=&quot;&quot; /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;"><strong>aria-labell</strong></span><span style="color: lightgreen;"> is not a valid ARIA attribute.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;input className=&quot;&quot; aria-labell=&quot;&quot; /&gt;
   <strong>   │ </strong>                    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the invalid </span><span style="color: lightgreen;"><strong>aria-*</strong></span><span style="color: lightgreen;"> attribute.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">                Check the list of all </span><span style="color: lightgreen;"><a href="https://developer.mozilla.org/en-US/docs/web/Accessibility/ARIA/Attributes#aria_attribute_types">valid</a></span><span style="color: lightgreen;"> aria-* attributes.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;input<span style="opacity: 0.8;">·</span>className=&quot;&quot;<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">i</span><span style="color: Tomato;">a</span><span style="color: Tomato;">-</span><span style="color: Tomato;">l</span><span style="color: Tomato;">a</span><span style="color: Tomato;">b</span><span style="color: Tomato;">e</span><span style="color: Tomato;">l</span><span style="color: Tomato;">l</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">&quot;</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>/&gt;
<strong>  </strong><strong>    │ </strong>                    <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>  
</code></pre>

```jsx
<div aria-lorem="foobar" />;
```

<pre class="language-text"><code class="language-text">a11y/useValidAriaProps.js:1:1 <a href="https://biomejs.dev/linter/rules/use-valid-aria-props">lint/a11y/useValidAriaProps</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The element contains invalid ARIA attribute(s)</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div aria-lorem=&quot;foobar&quot; /&gt;;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;"><strong>aria-lorem</strong></span><span style="color: lightgreen;"> is not a valid ARIA attribute.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div aria-lorem=&quot;foobar&quot; /&gt;;
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the invalid </span><span style="color: lightgreen;"><strong>aria-*</strong></span><span style="color: lightgreen;"> attribute.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">                Check the list of all </span><span style="color: lightgreen;"><a href="https://developer.mozilla.org/en-US/docs/web/Accessibility/ARIA/Attributes#aria_attribute_types">valid</a></span><span style="color: lightgreen;"> aria-* attributes.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;div<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">i</span><span style="color: Tomato;">a</span><span style="color: Tomato;">-</span><span style="color: Tomato;">l</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;">e</span><span style="color: Tomato;">m</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">b</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">&quot;</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>/&gt;;
<strong>  </strong><strong>    │ </strong>     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>   
</code></pre>

## Accessibility guidelines

- [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
