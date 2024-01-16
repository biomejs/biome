---
title: noPositiveTabindex (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/noPositiveTabindex`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/tabindex-no-positive.md" target="_blank"><code>tabindex-no-positive</code></a>

Prevent the usage of positive integers on `tabIndex` property

Avoid positive `tabIndex` property values to synchronize the flow of the page with keyboard tab order.

## Accessibility guidelines

[WCAG 2.4.3](https://www.w3.org/WAI/WCAG21/Understanding/focus-order)

## Examples

### Invalid

```jsx
<div tabIndex={1}>foo</div>
```

<pre class="language-text"><code class="language-text">a11y/noPositiveTabindex.js:1:15 <a href="https://biomejs.dev/linter/rules/no-positive-tabindex">lint/a11y/noPositiveTabindex</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid positive values for the </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> prop.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div tabIndex={1}&gt;foo&lt;/div&gt;
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Elements with a positive </span><span style="color: lightgreen;"><strong>tabIndex</strong></span><span style="color: lightgreen;"> override natural page content order. This causes elements without a positive tab index to come last when navigating using a keyboard.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use only 0 and -1 as </span><span style="color: lightgreen;"><strong>tabIndex</strong></span><span style="color: lightgreen;"> values. Avoid using </span><span style="color: lightgreen;"><strong>tabIndex</strong></span><span style="color: lightgreen;"> values greater than 0 and CSS properties that can change the order of focusable HTML elements.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Replace the </span><span style="color: lightgreen;"><strong>tableIndex</strong></span><span style="color: lightgreen;"> prop value with 0.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">v</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">t</span><span style="color: Tomato;">a</span><span style="color: Tomato;">b</span><span style="color: Tomato;">I</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">=</span><span style="color: Tomato;"><strong>{</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>}</strong></span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">/</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">v</span><span style="color: Tomato;">&gt;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">v</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">I</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><strong>&quot;</strong></span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;"><strong>&quot;</strong></span><span style="color: MediumSeaGreen;">&gt;</span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">&lt;</span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">v</span><span style="color: MediumSeaGreen;">&gt;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
<div tabIndex={"1"} />
```

<pre class="language-text"><code class="language-text">a11y/noPositiveTabindex.js:1:15 <a href="https://biomejs.dev/linter/rules/no-positive-tabindex">lint/a11y/noPositiveTabindex</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid positive values for the </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> prop.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div tabIndex={&quot;1&quot;} /&gt;
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Elements with a positive </span><span style="color: lightgreen;"><strong>tabIndex</strong></span><span style="color: lightgreen;"> override natural page content order. This causes elements without a positive tab index to come last when navigating using a keyboard.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use only 0 and -1 as </span><span style="color: lightgreen;"><strong>tabIndex</strong></span><span style="color: lightgreen;"> values. Avoid using </span><span style="color: lightgreen;"><strong>tabIndex</strong></span><span style="color: lightgreen;"> values greater than 0 and CSS properties that can change the order of focusable HTML elements.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Replace the </span><span style="color: lightgreen;"><strong>tableIndex</strong></span><span style="color: lightgreen;"> prop value with 0.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">v</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">t</span><span style="color: Tomato;">a</span><span style="color: Tomato;">b</span><span style="color: Tomato;">I</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">=</span><span style="color: Tomato;"><strong>{</strong></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;"><strong>}</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">/</span><span style="color: Tomato;">&gt;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">v</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">I</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;">&gt;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
React.createElement("div", { tabIndex: 1 })
```

<pre class="language-text"><code class="language-text">a11y/noPositiveTabindex.js:1:40 <a href="https://biomejs.dev/linter/rules/no-positive-tabindex">lint/a11y/noPositiveTabindex</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid positive values for the </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> prop.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>React.createElement(&quot;div&quot;, { tabIndex: 1 })
   <strong>   │ </strong>                                       <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Elements with a positive </span><span style="color: lightgreen;"><strong>tabIndex</strong></span><span style="color: lightgreen;"> override natural page content order. This causes elements without a positive tab index to come last when navigating using a keyboard.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use only 0 and -1 as </span><span style="color: lightgreen;"><strong>tabIndex</strong></span><span style="color: lightgreen;"> values. Avoid using </span><span style="color: lightgreen;"><strong>tabIndex</strong></span><span style="color: lightgreen;"> values greater than 0 and CSS properties that can change the order of focusable HTML elements.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Replace the </span><span style="color: lightgreen;"><strong>tableIndex</strong></span><span style="color: lightgreen;"> prop value with 0.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">R</span><span style="color: Tomato;">e</span><span style="color: Tomato;">a</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span><span style="color: Tomato;">.</span><span style="color: Tomato;">c</span><span style="color: Tomato;">r</span><span style="color: Tomato;">e</span><span style="color: Tomato;">a</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">E</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;">(</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">v</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">,</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">t</span><span style="color: Tomato;">a</span><span style="color: Tomato;">b</span><span style="color: Tomato;">I</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">:</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">}</span><span style="color: Tomato;">)</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">R</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">.</span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">E</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">v</span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">,</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">I</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">:</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>&quot;</strong></span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;"><strong>&quot;</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">}</span><span style="color: MediumSeaGreen;">)</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
<div tabIndex="0" />
```

```jsx
React.createElement("div", { tabIndex: -1 })
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
