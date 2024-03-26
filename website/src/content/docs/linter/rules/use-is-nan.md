---
title: useIsNan (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/useIsNan`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/use-isnan" target="_blank"><code>use-isnan</code></a>

Require calls to `isNaN()` when checking for `NaN`.

In JavaScript, `NaN` is a special value of the `Number` type.
It’s used to represent any of the "not-a-number" values represented by the double-precision 64-bit format as specified by the IEEE Standard for Binary Floating-Point Arithmetic.

Because `NaN` is unique in JavaScript by not being equal to anything, including itself, the results of comparisons to `NaN` are confusing:

- `NaN` === `NaN` or `NaN` == `NaN` evaluate to false
- `NaN` !== `NaN` or `NaN` != `NaN` evaluate to true

Therefore, use `Number.isNaN()` or global `isNaN()` functions to test whether a value is `NaN`.

Note that `Number.isNaN()` and `isNaN()` [do not have the same behavior](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/isNaN#description).
When the argument to `isNaN()` is not a number, the value is first coerced to a number.
`Number.isNaN()` does not perform this coercion.
Therefore, it is a more reliable way to test whether a value is `NaN`.

## Examples

### Invalid

```jsx
123 == NaN
```

<pre class="language-text"><code class="language-text">correctness/useIsNan.js:1:1 <a href="https://biomejs.dev/linter/rules/use-is-nan">lint/correctness/useIsNan</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use the Number.isNaN function to compare with NaN.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>123 == NaN
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>Number.isNaN()</strong></span><span style="color: lightgreen;"> instead.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">1</span><span style="color: Tomato;">2</span><span style="color: Tomato;">3</span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>=</strong></span><span style="color: Tomato;"><strong>=</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>N</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>N</strong></span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>N</strong></span><span style="color: MediumSeaGreen;"><strong>u</strong></span><span style="color: MediumSeaGreen;"><strong>m</strong></span><span style="color: MediumSeaGreen;"><strong>b</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>i</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>N</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>N</strong></span><span style="color: MediumSeaGreen;"><strong>(</strong></span><span style="color: MediumSeaGreen;">1</span><span style="color: MediumSeaGreen;">2</span><span style="color: MediumSeaGreen;">3</span><span style="color: MediumSeaGreen;"><strong>)</strong></span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
123 != NaN
```

<pre class="language-text"><code class="language-text">correctness/useIsNan.js:1:1 <a href="https://biomejs.dev/linter/rules/use-is-nan">lint/correctness/useIsNan</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use the Number.isNaN function to compare with NaN.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>123 != NaN
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>Number.isNaN()</strong></span><span style="color: lightgreen;"> instead.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>2</strong></span><span style="color: Tomato;"><strong>3</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;">!</span><span style="color: Tomato;"><strong>=</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>N</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>N</strong></span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">!</span><span style="color: MediumSeaGreen;"><strong>N</strong></span><span style="color: MediumSeaGreen;"><strong>u</strong></span><span style="color: MediumSeaGreen;"><strong>m</strong></span><span style="color: MediumSeaGreen;"><strong>b</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>i</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>N</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>N</strong></span><span style="color: MediumSeaGreen;"><strong>(</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>2</strong></span><span style="color: MediumSeaGreen;"><strong>3</strong></span><span style="color: MediumSeaGreen;"><strong>)</strong></span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
switch(foo) { case (NaN): break; }
```

<pre class="language-text"><code class="language-text">correctness/useIsNan.js:1:20 <a href="https://biomejs.dev/linter/rules/use-is-nan">lint/correctness/useIsNan</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">'case NaN' can never match. Use Number.isNaN before the switch.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>switch(foo) { case (NaN): break; }
   <strong>   │ </strong>                   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
Number.NaN == "abc"
```

<pre class="language-text"><code class="language-text">correctness/useIsNan.js:1:1 <a href="https://biomejs.dev/linter/rules/use-is-nan">lint/correctness/useIsNan</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use the Number.isNaN function to compare with NaN.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>Number.NaN == &quot;abc&quot;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>Number.isNaN()</strong></span><span style="color: lightgreen;"> instead.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>N</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>N</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>N</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>=</strong></span><span style="color: Tomato;"><strong>=</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">a</span><span style="color: Tomato;">b</span><span style="color: Tomato;">c</span><span style="color: Tomato;">&quot;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>N</strong></span><span style="color: MediumSeaGreen;"><strong>u</strong></span><span style="color: MediumSeaGreen;"><strong>m</strong></span><span style="color: MediumSeaGreen;"><strong>b</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>i</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>N</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>N</strong></span><span style="color: MediumSeaGreen;"><strong>(</strong></span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;"><strong>)</strong></span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
if (Number.isNaN(123) !== true) {}

foo(Number.NaN / 2)

switch(foo) {}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
