---
title: Lint Rule useAriaPropTypes
parent: lint/rules/index
---

# useAriaPropTypes (since v12.0.0)

Enforce that ARIA state and property values are valid.

## Examples

### Invalid

```jsx
<span role="checkbox" aria-checked="test">some text</span>
```

<pre class="language-text"><code class="language-text">nursery/useAriaPropTypes.js:1:23 <a href="https://biomejs.dev/lint/rules/useAriaPropTypes">lint/nursery/useAriaPropTypes</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The value of the ARIA attribute </span><span style="color: Orange;"><strong>aria-checked</strong></span><span style="color: Orange;"> is not correct.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;span role=&quot;checkbox&quot; aria-checked=&quot;test&quot;&gt;some text&lt;/span&gt;
   <strong>   │ </strong>                      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The only supported value for the </span><span style="color: rgb(38, 148, 255);"><strong>aria-checked</strong></span><span style="color: rgb(38, 148, 255);"> property one of the following:</span>
  
  - true
  - false
  - mixed
  
</code></pre>

```jsx
<span aria-labelledby="">some text</span>
```

<pre class="language-text"><code class="language-text">nursery/useAriaPropTypes.js:1:7 <a href="https://biomejs.dev/lint/rules/useAriaPropTypes">lint/nursery/useAriaPropTypes</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The value of the ARIA attribute </span><span style="color: Orange;"><strong>aria-labelledby</strong></span><span style="color: Orange;"> is not correct.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;span aria-labelledby=&quot;&quot;&gt;some text&lt;/span&gt;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The only supported value is text.</span>
  
</code></pre>

```jsx
<span aria-valuemax="hey">some text</span>
```

<pre class="language-text"><code class="language-text">nursery/useAriaPropTypes.js:1:7 <a href="https://biomejs.dev/lint/rules/useAriaPropTypes">lint/nursery/useAriaPropTypes</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The value of the ARIA attribute </span><span style="color: Orange;"><strong>aria-valuemax</strong></span><span style="color: Orange;"> is not correct.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;span aria-valuemax=&quot;hey&quot;&gt;some text&lt;/span&gt;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The only supported value is number.</span>
  
</code></pre>

```jsx
<span aria-orientation="hey">some text</span>
```

<pre class="language-text"><code class="language-text">nursery/useAriaPropTypes.js:1:7 <a href="https://biomejs.dev/lint/rules/useAriaPropTypes">lint/nursery/useAriaPropTypes</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The value of the ARIA attribute </span><span style="color: Orange;"><strong>aria-orientation</strong></span><span style="color: Orange;"> is not correct.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;span aria-orientation=&quot;hey&quot;&gt;some text&lt;/span&gt;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The only supported value for the </span><span style="color: rgb(38, 148, 255);"><strong>aria-orientation</strong></span><span style="color: rgb(38, 148, 255);"> property is one of the following:</span>
  
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
