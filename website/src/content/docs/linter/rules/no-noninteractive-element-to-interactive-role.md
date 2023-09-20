---
title: noNoninteractiveElementToInteractiveRole (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/noNoninteractiveElementToInteractiveRole`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Enforce that interactive ARIA roles are not assigned to non-interactive HTML elements.

Non-interactive HTML elements indicate _content_ and _containers_ in the user interface.
Non-interactive elements include `<main>`, `<area>`, `<h1>` (,`<h2>`, etc), `<img>`, `<li>`, `<ul>` and `<ol>`.

Interactive HTML elements indicate _controls_ in the user interface.
Interactive elements include `<a href>`, `<button>`, `<input>`, `<select>`, `<textarea>`.

[WAI-ARIA roles](https://www.w3.org/TR/wai-aria-1.1/#usage_intro) should not be used to convert a non-interactive element to an interactive element.
Interactive ARIA roles include `button`, `link`, `checkbox`, `menuitem`, `menuitemcheckbox`, `menuitemradio`, `option`, `radio`, `searchbox`, `switch` and `textbox`.

## Examples

### Invalid

```jsx
<h1 role="button">Some text</h1>
```

<pre class="language-text"><code class="language-text">a11y/noNoninteractiveElementToInteractiveRole.js:1:5 <a href="https://biomejs.dev/linter/rules/no-noninteractive-element-to-interactive-role">lint/a11y/noNoninteractiveElementToInteractiveRole</a> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The HTML element </span><span style="color: Tomato;"><strong>h1</strong></span><span style="color: Tomato;"> is non-interactive and should not have an interactive role.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;h1 role=&quot;button&quot;&gt;Some text&lt;/h1&gt;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Replace </span><span style="color: rgb(38, 148, 255);"><strong>h1</strong></span><span style="color: rgb(38, 148, 255);"> with a div or a span.</span>
  
</code></pre>

### Valid

```jsx
<span role="button">Some text</span>
```

## Accessibility guidelines

- [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)

### Resources

- [WAI-ARIA roles](https://www.w3.org/TR/wai-aria-1.1/#usage_intro)
- [WAI-ARIA Authoring Practices Guide - Design Patterns and Widgets](https://www.w3.org/TR/wai-aria-practices-1.1/#aria_ex)
- [Fundamental Keyboard Navigation Conventions](https://www.w3.org/TR/wai-aria-practices-1.1/#kbd_generalnav)
- [Mozilla Developer Network - ARIA Techniques](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/ARIA_Techniques/Using_the_button_role#Keyboard_and_focus)

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
