---
title: noGlobalIsNan (since v1.0.0)
---


Use `Number.isNaN` instead of global `isNaN`.

`Number.isNaN()` and `isNaN()` [have not the same behavior](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/isNaN#description).
When the argument to `isNaN()` is not a number, the value is first coerced to a number.
`Number.isNaN()` does not perform this coercion.
Therefore, it is a more reliable way to test whether a value is `NaN`.

## Examples

### Invalid

```jsx
isNaN({}); // true
```

<pre class="language-text"><code class="language-text">nursery/noGlobalIsNan.js:1:1 <a href="https://biomejs.dev/lint/rules/noGlobalIsNan">lint/nursery/noGlobalIsNan</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>isNaN</strong></span><span style="color: Tomato;"> is unsafe. It attempts a type coercion. Use </span><span style="color: Tomato;"><strong>Number.isNaN</strong></span><span style="color: Tomato;"> instead.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>isNaN({}); // true
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/isNaN#description">the MDN documentation</a></span><span style="color: rgb(38, 148, 255);"> for more details.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use </span><span style="color: rgb(38, 148, 255);"><strong>Number.isNaN</strong></span><span style="color: rgb(38, 148, 255);"> instead.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>N</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>N</strong></span><span style="color: Tomato;">(</span><span style="color: Tomato;">{</span><span style="color: Tomato;">}</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">/</span><span style="color: Tomato;">/</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">t</span><span style="color: Tomato;">r</span><span style="color: Tomato;">u</span><span style="color: Tomato;">e</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>N</strong></span><span style="color: MediumSeaGreen;"><strong>u</strong></span><span style="color: MediumSeaGreen;"><strong>m</strong></span><span style="color: MediumSeaGreen;"><strong>b</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>i</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>N</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>N</strong></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;">}</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">e</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

## Valid

```jsx
Number.isNaN({}); // false
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
