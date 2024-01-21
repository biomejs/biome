---
title: noNoninteractiveTabindex (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/noNoninteractiveTabindex`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-noninteractive-tabindex.md" target="_blank"><code>no-noninteractive-tabindex</code></a>

Enforce that `tabIndex` is not assigned to non-interactive HTML elements.

When using the tab key to navigate a webpage, limit it to interactive elements.
You don't need to add tabindex to items in an unordered list as assistive technology can navigate through the HTML.
Keep the tab ring small, which is the order of elements when tabbing, for a more efficient and accessible browsing experience.

## Examples

### Invalid

```jsx
<div tabIndex="0" />
```

<pre class="language-text"><code class="language-text">a11y/noNoninteractiveTabindex.js:1:6 <a href="https://biomejs.dev/linter/rules/no-noninteractive-tabindex">lint/a11y/noNoninteractiveTabindex</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The HTML element </span><span style="color: Tomato;"><strong>div</strong></span><span style="color: Tomato;"> is non-interactive. Do not use </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div tabIndex=&quot;0&quot; /&gt;
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Adding non-interactive elements to the keyboard navigation flow can confuse users.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the </span><span style="color: lightgreen;"><strong>tabIndex</strong></span><span style="color: lightgreen;"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;div<span style="opacity: 0.8;">·</span><span style="color: Tomato;">t</span><span style="color: Tomato;">a</span><span style="color: Tomato;">b</span><span style="color: Tomato;">I</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">0</span><span style="color: Tomato;">&quot;</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>/&gt;
<strong>  </strong><strong>    │ </strong>     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>  
</code></pre>

```jsx
<div role="article" tabIndex="0" />
```

<pre class="language-text"><code class="language-text">a11y/noNoninteractiveTabindex.js:1:21 <a href="https://biomejs.dev/linter/rules/no-noninteractive-tabindex">lint/a11y/noNoninteractiveTabindex</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The HTML element </span><span style="color: Tomato;"><strong>div</strong></span><span style="color: Tomato;"> is non-interactive. Do not use </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div role=&quot;article&quot; tabIndex=&quot;0&quot; /&gt;
   <strong>   │ </strong>                    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Adding non-interactive elements to the keyboard navigation flow can confuse users.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the </span><span style="color: lightgreen;"><strong>tabIndex</strong></span><span style="color: lightgreen;"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;div<span style="opacity: 0.8;">·</span>role=&quot;article&quot;<span style="opacity: 0.8;">·</span><span style="color: Tomato;">t</span><span style="color: Tomato;">a</span><span style="color: Tomato;">b</span><span style="color: Tomato;">I</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">0</span><span style="color: Tomato;">&quot;</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>/&gt;
<strong>  </strong><strong>    │ </strong>                    <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>  
</code></pre>

```jsx
<article tabIndex="0" />
```

<pre class="language-text"><code class="language-text">a11y/noNoninteractiveTabindex.js:1:10 <a href="https://biomejs.dev/linter/rules/no-noninteractive-tabindex">lint/a11y/noNoninteractiveTabindex</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The HTML element </span><span style="color: Tomato;"><strong>article</strong></span><span style="color: Tomato;"> is non-interactive. Do not use </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;article tabIndex=&quot;0&quot; /&gt;
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Adding non-interactive elements to the keyboard navigation flow can confuse users.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the </span><span style="color: lightgreen;"><strong>tabIndex</strong></span><span style="color: lightgreen;"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;article<span style="opacity: 0.8;">·</span><span style="color: Tomato;">t</span><span style="color: Tomato;">a</span><span style="color: Tomato;">b</span><span style="color: Tomato;">I</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">0</span><span style="color: Tomato;">&quot;</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>/&gt;
<strong>  </strong><strong>    │ </strong>         <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>  
</code></pre>

### Valid

```jsx
<div />
```

```jsx
<MyButton tabIndex={0} />
```

```jsx
<article tabIndex="-1" />
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
