---
title: noInteractiveElementToNoninteractiveRole (since v1.3.0)
---

**Diagnostic Category: `lint/nursery/noInteractiveElementToNoninteractiveRole`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Enforce that non-interactive ARIA roles are not assigned to interactive HTML elements.

Interactive HTML elements indicate controls in the user interface.
Interactive elements include `<a href>`, `<button>`, `<input>`, `<select>`, `<textarea>`.
Non-interactive HTML elements and non-interactive ARIA roles indicate content and containers in the user interface.
Non-interactive elements include `<main>`, `<area>`, `<h1>` (,`<h2>`, etc), `<img>`, `<li>`, `<ul>` and `<ol>`.

[WAI-ARIA roles](https://www.w3.org/TR/wai-aria-1.1/#usage_intro) should not be used to convert an interactive element to a non-interactive element.
Non-interactive ARIA roles include `article`, `banner`, `complementary`, `img`, `listitem`, `main`, `region` and `tooltip`.

Source: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-interactive-element-to-noninteractive-role.md

## Examples

### Invalid

```jsx
<input role="img" />;
```

<pre class="language-text"><code class="language-text">nursery/noInteractiveElementToNoninteractiveRole.js:1:1 <a href="https://biomejs.dev/linter/rules/no-interactive-element-to-noninteractive-role">lint/nursery/noInteractiveElementToNoninteractiveRole</a> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Interactive elements should not be assigned non-interactive roles.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;input role=&quot;img&quot; /&gt;;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">WAI-ARIA roles should not be used to convert an interactive element to a non-interactive element.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Wrap your interactive element in a &lt;div&gt; with the desired role or put the content inside your interactive element.</span>
  
</code></pre>

## Valid

```jsx
<input role="button" />;
```

### Options

```json
{
    "//": "...",
    "options": {
        "elementAndRoles": [
            { "element": "span", "roles": ["button", "grid"] },
            { "element": "div", "roles": ["button"] }
        ]
    }
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
