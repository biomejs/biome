---
title: Lint Rule useIsArray
parent: lint/rules/index
---

# useIsArray (since v1.0.0)

Use `Array.isArray()` instead of `instanceof Array`.

In _JavaScript_ some array-like objects such as _arguments_ are not instances of the `Array` class.    ///
Moreover, the global `Array` class can be different between two execution contexts.
For instance, two frames in a web browser have a distinct `Array` class.
Passing arrays across these contexts, results in arrays that are not instances of the contextual global `Array` class.
To avoid these issues, use `Array.isArray()` instead of `instanceof Array`.
See the [MDN docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/isArray) for more details.

Source: https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/no-instanceof-array.md

## Examples

### Invalid

```jsx
const xs = [];
if (xs instanceof Array) {}
```

<pre class="language-text"><code class="language-text">nursery/useIsArray.js:2:5 <a href="https://biomejs.dev/lint/rules/useIsArray">lint/nursery/useIsArray</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>Array.isArray()</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>instanceof Array</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>const xs = [];
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>if (xs instanceof Array) {}
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);"><strong>instanceof Array</strong></span><span style="color: rgb(38, 148, 255);"> returns false for array-like objects and arrays from other execution contexts.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use </span><span style="color: rgb(38, 148, 255);"><strong>Array.isArray()</strong></span><span style="color: rgb(38, 148, 255);"> instead.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  const xs = [];
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">i</span><span style="color: Tomato;">f</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;">x</span><span style="color: Tomato;">s</span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>f</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>A</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>y</strong></span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span><span style="color: Tomato;">}</span>
      <strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;"><strong>A</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>y</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>i</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>A</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>y</strong></span><span style="color: MediumSeaGreen;"><strong>(</strong></span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><strong>)</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;">}</span>
    <strong>3</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>

## Valid

```jsx
const xs = [];
if (Array.isArray(xs)) {}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
