---
title: noStaticElementInteractions (since vnext)
---

**Diagnostic Category: `lint/nursery/noStaticElementInteractions`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Static elements should not be interactive.

Static HTML elements do not have semantic meaning. This is clear in the case of `<div>` and `<span>`. It is less so clear in the case of elements that seem semantic, but that do not have a semantic mapping in the accessibility layer. For example `<a>`, `<big>`, `<blockquote>`, `<footer>`, `<picture>`, `<strike>` and `<time>` -- to name a few -- have no semantic layer mapping. They are as void of meaning as `<div>`.

The [WAI-ARIA role attribute](https://www.w3.org/TR/wai-aria-1.1/#usage_intro) confers a semantic mapping to an element. The semantic value can then be expressed to a user via assistive technology.
In order to add interactivity such as a mouse or key event listener to a static element, that element must be given a role value as well.

Source: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-static-element-interactions.md#jsx-a11yno-static-element-interactions

## Examples

### Invalid

```jsx
<div onClick={() => {}} />;
```

<pre class="language-text"><code class="language-text">nursery/noStaticElementInteractions.js:1:1 <a href="https://biomejs.dev/linter/rules/no-static-element-interactions">lint/nursery/noStaticElementInteractions</a> ━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Static elements should not be interactive.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div onClick={() =&gt; {}} /&gt;;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">To add interactivity such as a mouse or key event listener to a static element, give the element an appropriate role value</span>
  
</code></pre>

## Valid

```jsx
<div role="button" onClick={() => {}}  />;
```

```jsx
<button onClick={() => {}} className="foo" />;
```

```jsx
<input type="text" onClick={() => {}} />;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
