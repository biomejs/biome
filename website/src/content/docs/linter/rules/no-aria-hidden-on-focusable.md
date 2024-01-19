---
title: noAriaHiddenOnFocusable (since v1.4.0)
---

**Diagnostic Category: `lint/a11y/noAriaHiddenOnFocusable`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-aria-hidden-on-focusable.md" target="_blank"><code>no-aria-hidden-on-focusable</code></a>

Enforce that aria-hidden="true" is not set on focusable elements.

`aria-hidden="true"` can be used to hide purely decorative content from screen reader users.
A focusable element with `aria-hidden="true"` can be reached by keyboard.
This can lead to confusion or unexpected behavior for screen reader users.

## Example

### Invalid

```jsx
<div aria-hidden="true" tabIndex="0" />
```

<pre class="language-text"><code class="language-text">a11y/noAriaHiddenOnFocusable.js:1:1 <a href="https://biomejs.dev/linter/rules/no-aria-hidden-on-focusable">lint/a11y/noAriaHiddenOnFocusable</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Disallow </span><span style="color: Tomato;"><strong>aria-hidden=&quot;true&quot;</strong></span><span style="color: Tomato;"> from being set on focusable elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div aria-hidden=&quot;true&quot; tabIndex=&quot;0&quot; /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;"><strong>aria-hidden</strong></span><span style="color: lightgreen;"> should not be set to </span><span style="color: lightgreen;"><strong>true</strong></span><span style="color: lightgreen;"> on focusable elements because this can lead to confusing behavior for screen reader users.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the aria-hidden attribute from the element.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;div<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">i</span><span style="color: Tomato;">a</span><span style="color: Tomato;">-</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">d</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">t</span><span style="color: Tomato;">r</span><span style="color: Tomato;">u</span><span style="color: Tomato;">e</span><span style="color: Tomato;">&quot;</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>tabIndex=&quot;0&quot;<span style="opacity: 0.8;">·</span>/&gt;
<strong>  </strong><strong>    │ </strong>     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>               
</code></pre>

```jsx
<a href="/" aria-hidden="true" />
```

<pre class="language-text"><code class="language-text">a11y/noAriaHiddenOnFocusable.js:1:1 <a href="https://biomejs.dev/linter/rules/no-aria-hidden-on-focusable">lint/a11y/noAriaHiddenOnFocusable</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Disallow </span><span style="color: Tomato;"><strong>aria-hidden=&quot;true&quot;</strong></span><span style="color: Tomato;"> from being set on focusable elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a href=&quot;/&quot; aria-hidden=&quot;true&quot; /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;"><strong>aria-hidden</strong></span><span style="color: lightgreen;"> should not be set to </span><span style="color: lightgreen;"><strong>true</strong></span><span style="color: lightgreen;"> on focusable elements because this can lead to confusing behavior for screen reader users.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the aria-hidden attribute from the element.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;a<span style="opacity: 0.8;">·</span>href=&quot;/&quot;<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">i</span><span style="color: Tomato;">a</span><span style="color: Tomato;">-</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">d</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">t</span><span style="color: Tomato;">r</span><span style="color: Tomato;">u</span><span style="color: Tomato;">e</span><span style="color: Tomato;">&quot;</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>/&gt;
<strong>  </strong><strong>    │ </strong>            <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>  
</code></pre>

### Valid

```jsx
<button aria-hidden="true" tabIndex="-1" />
```

```jsx
<div aria-hidden="true"><a href="#"></a></div>
```

## Resources

- [aria-hidden elements do not contain focusable elements](https://dequeuniversity.com/rules/axe/html/4.4/aria-hidden-focus)
- [Element with aria-hidden has no content in sequential focus navigation](https://www.w3.org/WAI/standards-guidelines/act/rules/6cfa84/proposed/)
- [MDN aria-hidden](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden)

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
