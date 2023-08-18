---
title: Lint Rule noHeaderScope
parent: lint/rules/index
---

# noHeaderScope (since v11.0.0)

> This rule is recommended by Biome.

The scope prop should be used only on `<th>` elements.

## Examples

### Invalid

```jsx
<div scope={scope} />
```

<pre class="language-text"><code class="language-text">a11y/noHeaderScope.js:1:6 <a href="https://biomejs.dev/lint/rules/noHeaderScope">lint/a11y/noHeaderScope</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>scope</strong></span><span style="color: Tomato;"> attribute on elements other than </span><span style="color: Tomato;"><strong>th</strong></span><span style="color: Tomato;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div scope={scope} /&gt;
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the </span><span style="color: rgb(38, 148, 255);"><strong>scope</strong></span><span style="color: rgb(38, 148, 255);"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;div<span style="opacity: 0.8;">·</span><span style="color: Tomato;">s</span><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">p</span><span style="color: Tomato;">e</span><span style="color: Tomato;">=</span><span style="color: Tomato;">{</span><span style="color: Tomato;">s</span><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">p</span><span style="color: Tomato;">e</span><span style="color: Tomato;">}</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>/&gt;
<strong>  </strong><strong>    │ </strong>     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>  
</code></pre>

```jsx
<div scope="col" />
```

<pre class="language-text"><code class="language-text">a11y/noHeaderScope.js:1:6 <a href="https://biomejs.dev/lint/rules/noHeaderScope">lint/a11y/noHeaderScope</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>scope</strong></span><span style="color: Tomato;"> attribute on elements other than </span><span style="color: Tomato;"><strong>th</strong></span><span style="color: Tomato;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div scope=&quot;col&quot; /&gt;
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the </span><span style="color: rgb(38, 148, 255);"><strong>scope</strong></span><span style="color: rgb(38, 148, 255);"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;div<span style="opacity: 0.8;">·</span><span style="color: Tomato;">s</span><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">p</span><span style="color: Tomato;">e</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">l</span><span style="color: Tomato;">&quot;</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>/&gt;
<strong>  </strong><strong>    │ </strong>     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>  
</code></pre>

### Valid

```jsx
<th scope={scope}></th>
```

```jsx
<th scope="col"></th>
```

## Accessibility guidelines

- [WCAG 1.3.1](https://www.w3.org/WAI/WCAG21/Understanding/info-and-relationships)
- [WCAG 4.1.1](https://www.w3.org/WAI/WCAG21/Understanding/parsing)

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
