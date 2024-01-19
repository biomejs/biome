---
title: useConst (since v1.0.0)
---

**Diagnostic Category: `lint/style/useConst`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/prefer-const" target="_blank"><code>prefer-const</code></a>

Require `const` declarations for variables that are never reassigned after declared.

## Examples

### Invalid

```jsx
let a = 3;
console.log(a);
```

<pre class="language-text"><code class="language-text">style/useConst.js:1:1 <a href="https://biomejs.dev/linter/rules/use-const">lint/style/useConst</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>let</strong></span><span style="color: Tomato;"> declares a variable which is never re-assigned.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let a = 3;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>console.log(a);
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'a' is never re-assigned.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let a = 3;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>console.log(a);
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>const</strong></span><span style="color: lightgreen;"> instead.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">3</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>c</strong></span><span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;"><strong>n</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>t</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">3</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  console.log(a);
    <strong>3</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>

```jsx
// `a` is redefined (not reassigned) on each loop step.
for (let a of [1, 2, 3]) {
    console.log(a);
}
```

<pre class="language-text"><code class="language-text">style/useConst.js:2:6 <a href="https://biomejs.dev/linter/rules/use-const">lint/style/useConst</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>let</strong></span><span style="color: Tomato;"> declares a variable which is never re-assigned.</span>
  
    <strong>1 │ </strong>// `a` is redefined (not reassigned) on each loop step.
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>for (let a of [1, 2, 3]) {
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>    console.log(a);
    <strong>4 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'a' is never re-assigned.</span>
  
    <strong>1 │ </strong>// `a` is redefined (not reassigned) on each loop step.
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>for (let a of [1, 2, 3]) {
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>    console.log(a);
    <strong>4 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>const</strong></span><span style="color: lightgreen;"> instead.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  // `a` is redefined (not reassigned) on each loop step.
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">o</span><span style="color: Tomato;">f</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">[</span><span style="color: Tomato;">1</span><span style="color: Tomato;">,</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">2</span><span style="color: Tomato;">,</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">3</span><span style="color: Tomato;">]</span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span>
      <strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;"><strong>c</strong></span><span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;"><strong>n</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>t</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">[</span><span style="color: MediumSeaGreen;">1</span><span style="color: MediumSeaGreen;">,</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">2</span><span style="color: MediumSeaGreen;">,</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">3</span><span style="color: MediumSeaGreen;">]</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span>
    <strong>3</strong> <strong>3</strong><strong> │ </strong>      console.log(a);
    <strong>4</strong> <strong>4</strong><strong> │ </strong>  }
  
</code></pre>

```jsx
// `a` is redefined (not reassigned) on each loop step.
for (let a in [1, 2, 3]) {
    console.log(a);
}
```

<pre class="language-text"><code class="language-text">style/useConst.js:2:6 <a href="https://biomejs.dev/linter/rules/use-const">lint/style/useConst</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>let</strong></span><span style="color: Tomato;"> declares a variable which is never re-assigned.</span>
  
    <strong>1 │ </strong>// `a` is redefined (not reassigned) on each loop step.
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>for (let a in [1, 2, 3]) {
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>    console.log(a);
    <strong>4 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'a' is never re-assigned.</span>
  
    <strong>1 │ </strong>// `a` is redefined (not reassigned) on each loop step.
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>for (let a in [1, 2, 3]) {
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>    console.log(a);
    <strong>4 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>const</strong></span><span style="color: lightgreen;"> instead.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  // `a` is redefined (not reassigned) on each loop step.
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">[</span><span style="color: Tomato;">1</span><span style="color: Tomato;">,</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">2</span><span style="color: Tomato;">,</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">3</span><span style="color: Tomato;">]</span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span>
      <strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;"><strong>c</strong></span><span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;"><strong>n</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>t</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">[</span><span style="color: MediumSeaGreen;">1</span><span style="color: MediumSeaGreen;">,</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">2</span><span style="color: MediumSeaGreen;">,</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">3</span><span style="color: MediumSeaGreen;">]</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span>
    <strong>3</strong> <strong>3</strong><strong> │ </strong>      console.log(a);
    <strong>4</strong> <strong>4</strong><strong> │ </strong>  }
  
</code></pre>

```jsx
let a = 3;
{
    let a = 4;
    a = 2;
}
```

<pre class="language-text"><code class="language-text">style/useConst.js:1:1 <a href="https://biomejs.dev/linter/rules/use-const">lint/style/useConst</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>let</strong></span><span style="color: Tomato;"> declares a variable which is never re-assigned.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let a = 3;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>{
    <strong>3 │ </strong>    let a = 4;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'a' is never re-assigned.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let a = 3;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>{
    <strong>3 │ </strong>    let a = 4;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>const</strong></span><span style="color: lightgreen;"> instead.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">3</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>c</strong></span><span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;"><strong>n</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>t</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">3</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  {
    <strong>3</strong> <strong>3</strong><strong> │ </strong>      let a = 4;
  
</code></pre>

### Valid

```jsx
let a = 2;
a = 3;
console.log(a);
```

```jsx
let a = 1, b = 2;
b = 3;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
