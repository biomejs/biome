---
title: useAriaPropsForRole (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/useAriaPropsForRole`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Enforce that elements with ARIA roles must have all required ARIA attributes for that role.

## Examples

### Invalid

```jsx
<span role="checkbox"></span>
```

<pre class="language-text"><code class="language-text">a11y/useAriaPropsForRole.js:1:7 <a href="https://biomejs.dev/linter/rules/use-aria-props-for-role">lint/a11y/useAriaPropsForRole</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The element with the </span><span style="color: Tomato;"><strong>checkbox</strong></span><span style="color: Tomato;"> ARIA role does not have the required ARIA attributes.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;span role=&quot;checkbox&quot;&gt;&lt;/span&gt;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Missing ARIA prop(s):</span>
  
  - aria-checked
  
</code></pre>

```jsx
<span role="heading"></span>
```

<pre class="language-text"><code class="language-text">a11y/useAriaPropsForRole.js:1:7 <a href="https://biomejs.dev/linter/rules/use-aria-props-for-role">lint/a11y/useAriaPropsForRole</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The element with the </span><span style="color: Tomato;"><strong>heading</strong></span><span style="color: Tomato;"> ARIA role does not have the required ARIA attributes.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;span role=&quot;heading&quot;&gt;&lt;/span&gt;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Missing ARIA prop(s):</span>
  
  - aria-level
  
</code></pre>

### Valid

```jsx
<span role="checkbox" aria-checked="true"></span>
```

```jsx
<span role="heading" aria-level="1"></span>
```

## Accessibility guidelines

- [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)

### Resources

- [ARIA Spec, Roles](https://www.w3.org/TR/wai-aria/#roles)
- [Chrome Audit Rules, AX_ARIA_03](https://github.com/GoogleChrome/accessibility-developer-tools/wiki/Audit-Rules#ax_aria_03)

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
