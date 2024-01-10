---
title: useValidAriaValues (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/useValidAriaValues`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-proptypes.md" target="_blank"><code>aria-proptypes</code></a>

Enforce that ARIA state and property values are valid.

## Examples

### Invalid

```jsx
<span role="checkbox" aria-checked="test">some text</span>
```

<pre class="language-text"><code class="language-text">a11y/useValidAriaValues.js:1:23 <a href="https://biomejs.dev/linter/rules/use-valid-aria-values">lint/a11y/useValidAriaValues</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The value of the ARIA attribute </span><span style="color: Tomato;"><strong>aria-checked</strong></span><span style="color: Tomato;"> is not correct.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;span role=&quot;checkbox&quot; aria-checked=&quot;test&quot;&gt;some text&lt;/span&gt;
   <strong>   │ </strong>                      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The only supported value for the </span><span style="color: lightgreen;"><strong>aria-checked</strong></span><span style="color: lightgreen;"> property one of the following:</span>
  
  - true
  - false
  - mixed
  
</code></pre>

```jsx
<span aria-labelledby="">some text</span>
```

<pre class="language-text"><code class="language-text">a11y/useValidAriaValues.js:1:7 <a href="https://biomejs.dev/linter/rules/use-valid-aria-values">lint/a11y/useValidAriaValues</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The value of the ARIA attribute </span><span style="color: Tomato;"><strong>aria-labelledby</strong></span><span style="color: Tomato;"> is not correct.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;span aria-labelledby=&quot;&quot;&gt;some text&lt;/span&gt;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The only supported value is text.</span>
  
</code></pre>

```jsx
<span aria-valuemax="hey">some text</span>
```

<pre class="language-text"><code class="language-text">a11y/useValidAriaValues.js:1:7 <a href="https://biomejs.dev/linter/rules/use-valid-aria-values">lint/a11y/useValidAriaValues</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The value of the ARIA attribute </span><span style="color: Tomato;"><strong>aria-valuemax</strong></span><span style="color: Tomato;"> is not correct.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;span aria-valuemax=&quot;hey&quot;&gt;some text&lt;/span&gt;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The only supported value is number.</span>
  
</code></pre>

```jsx
<span aria-orientation="hey">some text</span>
```

<pre class="language-text"><code class="language-text">a11y/useValidAriaValues.js:1:7 <a href="https://biomejs.dev/linter/rules/use-valid-aria-values">lint/a11y/useValidAriaValues</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The value of the ARIA attribute </span><span style="color: Tomato;"><strong>aria-orientation</strong></span><span style="color: Tomato;"> is not correct.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;span aria-orientation=&quot;hey&quot;&gt;some text&lt;/span&gt;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The only supported value for the </span><span style="color: lightgreen;"><strong>aria-orientation</strong></span><span style="color: lightgreen;"> property is one of the following:</span>
  
  - vertical
  - undefined
  - horizontal
  
</code></pre>

### Valid

```jsx
<>
    <span role="checkbox" aria-checked={checked} >some text</span>
    <span aria-labelledby="fooId barId" >some text</span>
</>
```

## Accessibility guidelines

- [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)

### Resources

- [ARIA Spec, States and Properties](https://www.w3.org/TR/wai-aria/#states_and_properties)
- [Chrome Audit Rules, AX_ARIA_04](https://github.com/GoogleChrome/accessibility-developer-tools/wiki/Audit-Rules#ax_aria_04)

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
