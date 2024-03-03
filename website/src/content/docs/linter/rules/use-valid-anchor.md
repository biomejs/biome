---
title: useValidAnchor (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/useValidAnchor`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/anchor-is-valid.md" target="_blank"><code>anchor-is-valid</code></a>

Enforce that all anchors are valid, and they are navigable elements.

The anchor element (`<a></a>`) - also called **hyperlink** - is an important element
that allows users to navigate pages, in the same page, same website or on another website.

While before it was possible to attach logic to an anchor element, with the advent of JSX libraries,
it's now  easier to attach logic to any HTML element, anchors included.

This rule is designed to prevent users from attaching logic at the click of anchors when the `href`
provided to the anchor element is not valid. Avoid using `#` symbol inside the `href` when you are
attaching the logic to the anchor element. If the anchor has logic attached to it with an incorrect `href`
the rules suggests to turn it to a `button`, because that's likely what the user wants.

Anchor `<a></a>` elements should be used for navigation, while `<button></button>` should be
used for user interaction.

There are **many reasons** why an anchor should not have a logic with an incorrect `href` attribute:

- it can disrupt the correct flow of the user navigation e.g. a user that wants to open the link
in another tab, but the default "click" behavior is prevented
- it can source of invalid links, and crawlers can't navigate the website, risking to penalize
SEO ranking

For a detailed explanation, check out https://marcysutton.com/links-vs-buttons-in-modern-web-applications

## Examples

### Invalid

```jsx
<a href={null}>navigate here</a>
```

<pre class="language-text"><code class="language-text">a11y/useValidAnchor.js:1:4 <a href="https://biomejs.dev/linter/rules/use-valid-anchor">lint/a11y/useValidAnchor</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a valid value for the attribute </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a href={null}&gt;navigate here&lt;/a&gt;
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The href attribute should be a valid a URL</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Check </span><span style="color: lightgreen;"><a href="https://marcysutton.com/links-vs-buttons-in-modern-web-applications">this thorough explanation</a></span><span style="color: lightgreen;"> to better understand the context.</span>
  
</code></pre>

```jsx
<a href={undefined}>navigate here</a>
```

<pre class="language-text"><code class="language-text">a11y/useValidAnchor.js:1:4 <a href="https://biomejs.dev/linter/rules/use-valid-anchor">lint/a11y/useValidAnchor</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a valid value for the attribute </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a href={undefined}&gt;navigate here&lt;/a&gt;
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The href attribute should be a valid a URL</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Check </span><span style="color: lightgreen;"><a href="https://marcysutton.com/links-vs-buttons-in-modern-web-applications">this thorough explanation</a></span><span style="color: lightgreen;"> to better understand the context.</span>
  
</code></pre>

```jsx
<a href>navigate here</a>
```

<pre class="language-text"><code class="language-text">a11y/useValidAnchor.js:1:4 <a href="https://biomejs.dev/linter/rules/use-valid-anchor">lint/a11y/useValidAnchor</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a valid value for the attribute </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a href&gt;navigate here&lt;/a&gt;
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The href attribute should be a valid a URL</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Check </span><span style="color: lightgreen;"><a href="https://marcysutton.com/links-vs-buttons-in-modern-web-applications">this thorough explanation</a></span><span style="color: lightgreen;"> to better understand the context.</span>
  
</code></pre>

```jsx
<a href="javascript:void(0)">navigate here</a>
```

<pre class="language-text"><code class="language-text">a11y/useValidAnchor.js:1:4 <a href="https://biomejs.dev/linter/rules/use-valid-anchor">lint/a11y/useValidAnchor</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a valid value for the attribute </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a href=&quot;javascript:void(0)&quot;&gt;navigate here&lt;/a&gt;
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The href attribute should be a valid a URL</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Check </span><span style="color: lightgreen;"><a href="https://marcysutton.com/links-vs-buttons-in-modern-web-applications">this thorough explanation</a></span><span style="color: lightgreen;"> to better understand the context.</span>
  
</code></pre>

```jsx
<a onClick={something}>navigate here</a>
```

<pre class="language-text"><code class="language-text">a11y/useValidAnchor.js:1:4 <a href="https://biomejs.dev/linter/rules/use-valid-anchor">lint/a11y/useValidAnchor</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a onClick={something}&gt;navigate here&lt;/a&gt;
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Anchor elements should only be used for default sections or page navigation</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Check </span><span style="color: lightgreen;"><a href="https://marcysutton.com/links-vs-buttons-in-modern-web-applications">this thorough explanation</a></span><span style="color: lightgreen;"> to better understand the context.</span>
  
</code></pre>

### Valid

```jsx
<a href="https://example.com" onClick={something}>navigate here</a>
```

```jsx
<a href={`https://www.javascript.com`}>navigate here</a>
```

```jsx
<a href={somewhere}>navigate here</a>
```

```jsx
<a {...spread}>navigate here</a>
```

## Accessibility guidelines

- [WCAG 2.1.1](https://www.w3.org/WAI/WCAG21/Understanding/keyboard)

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
