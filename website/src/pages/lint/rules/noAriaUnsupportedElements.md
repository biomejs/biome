---
title: Lint Rule noAriaUnsupportedElements
parent: lint/rules/index
---

# noAriaUnsupportedElements (since v12.1.0)

Enforce that elements that do not support ARIA roles, states, and properties do not have those attributes.

Source: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-unsupported-elements.md

## Examples

### Invalid

```jsx
<meta charset="UTF-8" role="meta" />
```

<pre class="language-text"><code class="language-text">nursery/noAriaUnsupportedElements.js:1:1 <a href="https://biomejs.dev/lint/rules/noAriaUnsupportedElements">lint/nursery/noAriaUnsupportedElements</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>role</strong></span><span style="color: Tomato;"> attribute and </span><span style="color: Tomato;"><strong>aria-*</strong></span><span style="color: Tomato;"> attributes when using </span><span style="color: Tomato;"><strong>meta</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>html</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>script</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;meta charset=&quot;UTF-8&quot; role=&quot;meta&quot; /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Using role on elements that do not support them can cause issues with screen readers.</span>
  
</code></pre>

```jsx
<html aria-required="true" />
```

<pre class="language-text"><code class="language-text">nursery/noAriaUnsupportedElements.js:1:1 <a href="https://biomejs.dev/lint/rules/noAriaUnsupportedElements">lint/nursery/noAriaUnsupportedElements</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>role</strong></span><span style="color: Tomato;"> attribute and </span><span style="color: Tomato;"><strong>aria-*</strong></span><span style="color: Tomato;"> attributes when using </span><span style="color: Tomato;"><strong>meta</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>html</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>script</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;html aria-required=&quot;true&quot; /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Using aria-* on elements that do not support them can cause issues with screen readers.</span>
  
</code></pre>

## Valid

```jsx
<meta charset="UTF-8" />
```

```jsx
<html></html>
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
