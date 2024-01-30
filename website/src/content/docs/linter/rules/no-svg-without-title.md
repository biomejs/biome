---
title: noSvgWithoutTitle (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/noSvgWithoutTitle`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Enforces the usage of the `title` element for the `svg` element.

It is not possible to specify the `alt` attribute for the `svg` as for the `img`.
To make svg accessible, the following methods are available:

- provide the `title` element as the first child to `svg`
- provide `role="img"` and `aria-label` or `aria-labelledby` to `svg`

## Examples

### Invalid

```jsx
<svg>foo</svg>
```

<pre class="language-text"><code class="language-text">a11y/noSvgWithoutTitle.js:1:1 <a href="https://biomejs.dev/linter/rules/no-svg-without-title">lint/a11y/noSvgWithoutTitle</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Alternative text </span><span style="color: Tomato;"><strong>title</strong></span><span style="color: Tomato;"> element cannot be empty</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;svg&gt;foo&lt;/svg&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">For accessibility purposes, </span><span style="color: lightgreen;"><strong>SVGs</strong></span><span style="color: lightgreen;"> should have an alternative text, provided via </span><span style="color: lightgreen;"><strong>title</strong></span><span style="color: lightgreen;"> element. If the svg element has role=&quot;img&quot;, you should add the </span><span style="color: lightgreen;"><strong>aria-label</strong></span><span style="color: lightgreen;"> or </span><span style="color: lightgreen;"><strong>aria-labelledby</strong></span><span style="color: lightgreen;"> attribute.</span>
  
</code></pre>

```jsx
<svg>
    <title></title>
    <circle />
</svg>
```

<pre class="language-text"><code class="language-text">a11y/noSvgWithoutTitle.js:1:1 <a href="https://biomejs.dev/linter/rules/no-svg-without-title">lint/a11y/noSvgWithoutTitle</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Alternative text </span><span style="color: Tomato;"><strong>title</strong></span><span style="color: Tomato;"> element cannot be empty</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;svg&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    &lt;title&gt;&lt;/title&gt;
    <strong>3 │ </strong>    &lt;circle /&gt;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">For accessibility purposes, </span><span style="color: lightgreen;"><strong>SVGs</strong></span><span style="color: lightgreen;"> should have an alternative text, provided via </span><span style="color: lightgreen;"><strong>title</strong></span><span style="color: lightgreen;"> element. If the svg element has role=&quot;img&quot;, you should add the </span><span style="color: lightgreen;"><strong>aria-label</strong></span><span style="color: lightgreen;"> or </span><span style="color: lightgreen;"><strong>aria-labelledby</strong></span><span style="color: lightgreen;"> attribute.</span>
  
</code></pre>

```jsx
<svg>foo</svg>
```

<pre class="language-text"><code class="language-text">a11y/noSvgWithoutTitle.js:1:1 <a href="https://biomejs.dev/linter/rules/no-svg-without-title">lint/a11y/noSvgWithoutTitle</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Alternative text </span><span style="color: Tomato;"><strong>title</strong></span><span style="color: Tomato;"> element cannot be empty</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;svg&gt;foo&lt;/svg&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">For accessibility purposes, </span><span style="color: lightgreen;"><strong>SVGs</strong></span><span style="color: lightgreen;"> should have an alternative text, provided via </span><span style="color: lightgreen;"><strong>title</strong></span><span style="color: lightgreen;"> element. If the svg element has role=&quot;img&quot;, you should add the </span><span style="color: lightgreen;"><strong>aria-label</strong></span><span style="color: lightgreen;"> or </span><span style="color: lightgreen;"><strong>aria-labelledby</strong></span><span style="color: lightgreen;"> attribute.</span>
  
</code></pre>

```jsx
<svg role="img" aria-label="">
    <span id="">Pass</span>
</svg>
```

### Valid

```jsx
<svg>
    <rect />
    <rect />
    <g>
        <circle />
        <circle />
        <g>
            <title>Pass</title>
            <circle />
            <circle />
        </g>
    </g>
</svg>
```

```jsx
<svg>
    <title>Pass</title>
    <circle />
</svg>
```

```jsx
<svg role="img" aria-labelledby="title">
    <span id="title">Pass</span>
</svg>
```

```jsx
<svg role="img" aria-label="title">
    <span id="title">Pass</span>
</svg>
```

## Accessibility guidelines

[Document Structure – SVG 1.1 (Second Edition)](https://www.w3.org/TR/SVG11/struct.html#DescriptionAndTitleElements)
[ARIA: img role - Accessibility | MDN](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/img_role)
[Accessible SVGs | CSS-Tricks - CSS-Tricks](https://css-tricks.com/accessible-svgs/)
[Contextually Marking up accessible images and SVGs | scottohara.me](https://www.scottohara.me/blog/2019/05/22/contextual-images-svgs-and-a11y.html)

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
