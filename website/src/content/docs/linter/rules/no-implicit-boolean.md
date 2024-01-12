---
title: noImplicitBoolean (since v1.0.0)
---

**Diagnostic Category: `lint/style/noImplicitBoolean`**

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/jsx-boolean-value.md" target="_blank"><code>jsx-boolean-value</code></a>

Disallow implicit `true` values on JSX boolean attributes

## Examples

### Invalid

```jsx
<input disabled />
```

<pre class="language-text"><code class="language-text">style/noImplicitBoolean.js:1:8 <a href="https://biomejs.dev/linter/rules/no-implicit-boolean">lint/style/noImplicitBoolean</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Use explicit boolean values for boolean JSX props.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;input disabled /&gt;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Add explicit `true` literal for this attribute</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;input<span style="opacity: 0.8;">·</span>disabled<span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">}</span><span style="opacity: 0.8;">·</span>/&gt;
<strong>  </strong><strong>    │ </strong>               <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>   
</code></pre>

### Valid

```jsx
<input disabled={false} />
```

```jsx
<input disabled={''} />
```

```jsx
<input disabled={0} />
```

```jsx
<input disabled={undefined} />
```

```jsx
<input disabled='false' />
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
