---
title: noPositiveTabindex (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/noPositiveTabindex`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Prevent the usage of positive integers on `tabIndex` property

Avoid positive `tabIndex` property values to synchronize the flow of the page with keyboard tab order.

## Accessibility guidelines

[WCAG 2.4.3](https://www.w3.org/WAI/WCAG21/Understanding/focus-order)

## Examples

### Invalid

```jsx
<div tabIndex={1}>foo</div>
```

<pre class="language-text"><code class="language-text">a11y/noPositiveTabindex.js:1:15 <a href="https://biomejs.dev/linter/rules/no-positive-tabindex">lint/a11y/noPositiveTabindex</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid positive values for the </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> prop.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div tabIndex={1}&gt;foo&lt;/div&gt;
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Elements with a positive </span><span style="color: rgb(38, 148, 255);"><strong>tabIndex</strong></span><span style="color: rgb(38, 148, 255);"> override natural page content order. This causes elements without a positive tab index to come last when navigating using a keyboard.</span>
  
</code></pre>

```jsx
<div tabIndex={"1"} />
```

<pre class="language-text"><code class="language-text">a11y/noPositiveTabindex.js:1:15 <a href="https://biomejs.dev/linter/rules/no-positive-tabindex">lint/a11y/noPositiveTabindex</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid positive values for the </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> prop.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div tabIndex={&quot;1&quot;} /&gt;
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Elements with a positive </span><span style="color: rgb(38, 148, 255);"><strong>tabIndex</strong></span><span style="color: rgb(38, 148, 255);"> override natural page content order. This causes elements without a positive tab index to come last when navigating using a keyboard.</span>
  
</code></pre>

```jsx
React.createElement("div", { tabIndex: 1 })
```

<pre class="language-text"><code class="language-text">a11y/noPositiveTabindex.js:1:40 <a href="https://biomejs.dev/linter/rules/no-positive-tabindex">lint/a11y/noPositiveTabindex</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid positive values for the </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> prop.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>React.createElement(&quot;div&quot;, { tabIndex: 1 })
   <strong>   │ </strong>                                       <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Elements with a positive </span><span style="color: rgb(38, 148, 255);"><strong>tabIndex</strong></span><span style="color: rgb(38, 148, 255);"> override natural page content order. This causes elements without a positive tab index to come last when navigating using a keyboard.</span>
  
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
