---
title: useAnchorContent (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/useAnchorContent`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/anchor-has-content.md" target="_blank"><code>anchor-has-content</code></a>

Enforce that anchors have content and that the content is accessible to screen readers.

Accessible means the content is not hidden using the `aria-hidden` attribute.
Refer to the references to learn about why this is important.

## Examples

### Invalid

```jsx
<a />
```

<pre class="language-text"><code class="language-text">a11y/useAnchorContent.js:1:1 <a href="https://biomejs.dev/linter/rules/use-anchor-content">lint/a11y/useAnchorContent</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>`a`</strong></span><span style="color: Tomato;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All links on a page should have content that is accessible to screen readers.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Accessible content refers to digital content that is designed and structured in a way that makes it easy for people with disabilities to access, understand, and interact with using assistive technologies.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Follow these links for more information,
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;"> </span><span style="color: lightgreen;"><a href="https://www.w3.org/WAI/WCAG21/Understanding/link-purpose-in-context">WCAG 2.4.4</a></span><span style="color: lightgreen;">
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;"> </span><span style="color: lightgreen;"><a href="https://www.w3.org/WAI/WCAG21/Understanding/name-role-value">WCAG 4.1.2</a></span>
  
</code></pre>

```jsx
<a></a>
```

<pre class="language-text"><code class="language-text">a11y/useAnchorContent.js:1:1 <a href="https://biomejs.dev/linter/rules/use-anchor-content">lint/a11y/useAnchorContent</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>`a`</strong></span><span style="color: Tomato;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a&gt;&lt;/a&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All links on a page should have content that is accessible to screen readers.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Accessible content refers to digital content that is designed and structured in a way that makes it easy for people with disabilities to access, understand, and interact with using assistive technologies.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Follow these links for more information,
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;"> </span><span style="color: lightgreen;"><a href="https://www.w3.org/WAI/WCAG21/Understanding/link-purpose-in-context">WCAG 2.4.4</a></span><span style="color: lightgreen;">
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;"> </span><span style="color: lightgreen;"><a href="https://www.w3.org/WAI/WCAG21/Understanding/name-role-value">WCAG 4.1.2</a></span>
  
</code></pre>

```jsx
<a>    </a>
```

<pre class="language-text"><code class="language-text">a11y/useAnchorContent.js:1:1 <a href="https://biomejs.dev/linter/rules/use-anchor-content">lint/a11y/useAnchorContent</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>`a`</strong></span><span style="color: Tomato;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a&gt;    &lt;/a&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All links on a page should have content that is accessible to screen readers.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Accessible content refers to digital content that is designed and structured in a way that makes it easy for people with disabilities to access, understand, and interact with using assistive technologies.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Follow these links for more information,
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;"> </span><span style="color: lightgreen;"><a href="https://www.w3.org/WAI/WCAG21/Understanding/link-purpose-in-context">WCAG 2.4.4</a></span><span style="color: lightgreen;">
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;"> </span><span style="color: lightgreen;"><a href="https://www.w3.org/WAI/WCAG21/Understanding/name-role-value">WCAG 4.1.2</a></span>
  
</code></pre>

```jsx
<a aria-hidden>content</a>
```

<pre class="language-text"><code class="language-text">a11y/useAnchorContent.js:1:1 <a href="https://biomejs.dev/linter/rules/use-anchor-content">lint/a11y/useAnchorContent</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>`a`</strong></span><span style="color: Tomato;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a aria-hidden&gt;content&lt;/a&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All links on a page should have content that is accessible to screen readers.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Accessible content refers to digital content that is designed and structured in a way that makes it easy for people with disabilities to access, understand, and interact with using assistive technologies.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Follow these links for more information,
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;"> </span><span style="color: lightgreen;"><a href="https://www.w3.org/WAI/WCAG21/Understanding/link-purpose-in-context">WCAG 2.4.4</a></span><span style="color: lightgreen;">
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;"> </span><span style="color: lightgreen;"><a href="https://www.w3.org/WAI/WCAG21/Understanding/name-role-value">WCAG 4.1.2</a></span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the </span><span style="color: lightgreen;"><strong>aria-hidden</strong></span><span style="color: lightgreen;"> attribute to allow the anchor element and its content visible to assistive technologies.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;a<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">i</span><span style="color: Tomato;">a</span><span style="color: Tomato;">-</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">d</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span>&gt;content&lt;/a&gt;
<strong>  </strong><strong>    │ </strong>   <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>            
</code></pre>

```jsx
<a><span aria-hidden="true">content</span></a>
```

<pre class="language-text"><code class="language-text">a11y/useAnchorContent.js:1:1 <a href="https://biomejs.dev/linter/rules/use-anchor-content">lint/a11y/useAnchorContent</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>`a`</strong></span><span style="color: Tomato;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a&gt;&lt;span aria-hidden=&quot;true&quot;&gt;content&lt;/span&gt;&lt;/a&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All links on a page should have content that is accessible to screen readers.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Accessible content refers to digital content that is designed and structured in a way that makes it easy for people with disabilities to access, understand, and interact with using assistive technologies.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Follow these links for more information,
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;"> </span><span style="color: lightgreen;"><a href="https://www.w3.org/WAI/WCAG21/Understanding/link-purpose-in-context">WCAG 2.4.4</a></span><span style="color: lightgreen;">
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;"> </span><span style="color: lightgreen;"><a href="https://www.w3.org/WAI/WCAG21/Understanding/name-role-value">WCAG 4.1.2</a></span>
  
</code></pre>

### Valid

```jsx
<a>content</a>
```

```jsx
function html() {
    return { __html: "foo" }
}
<a dangerouslySetInnerHTML={html()} />
```

```jsx
<a><TextWrapper aria-hidden={true} />content</a>
```

```jsx
<a><div aria-hidden="true"></div>content</a>
```

## Accessibility guidelines

- [WCAG 2.4.4](https://www.w3.org/WAI/WCAG21/Understanding/link-purpose-in-context)
- [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
