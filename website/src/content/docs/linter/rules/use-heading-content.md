---
title: useHeadingContent (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/useHeadingContent`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/heading-has-content.md" target="_blank"><code>heading-has-content</code></a>

Enforce that heading elements (h1, h2, etc.) have content and that the content is accessible to screen readers. Accessible means that it is not hidden using the aria-hidden prop.

## Examples

### Invalid

```jsx
<h1 />
```

<pre class="language-text"><code class="language-text">a11y/useHeadingContent.js:1:1 <a href="https://biomejs.dev/linter/rules/use-heading-content">lint/a11y/useHeadingContent</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>heading</strong></span><span style="color: Tomato;">  elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;h1 /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All headings on a page should have content that is accessible to screen readers.</span>
  
</code></pre>

```jsx
<h1><div aria-hidden /></h1>
```

<pre class="language-text"><code class="language-text">a11y/useHeadingContent.js:1:1 <a href="https://biomejs.dev/linter/rules/use-heading-content">lint/a11y/useHeadingContent</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>heading</strong></span><span style="color: Tomato;">  elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;h1&gt;&lt;div aria-hidden /&gt;&lt;/h1&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All headings on a page should have content that is accessible to screen readers.</span>
  
</code></pre>

```jsx
<h1></h1>
```

<pre class="language-text"><code class="language-text">a11y/useHeadingContent.js:1:1 <a href="https://biomejs.dev/linter/rules/use-heading-content">lint/a11y/useHeadingContent</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>heading</strong></span><span style="color: Tomato;">  elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;h1&gt;&lt;/h1&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All headings on a page should have content that is accessible to screen readers.</span>
  
</code></pre>

### Valid

```jsx
<h1>heading</h1>
```

```jsx
<h1><div aria-hidden="true"></div>visible content</h1>
```

```jsx
<h1 dangerouslySetInnerHTML={{ __html: "heading" }} />
```

```jsx
<h1><div aria-hidden />visible content</h1>
```

## Accessibility guidelines

- [WCAG 2.4.6](https://www.w3.org/TR/UNDERSTANDING-WCAG20/navigation-mechanisms-descriptive.html)

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
