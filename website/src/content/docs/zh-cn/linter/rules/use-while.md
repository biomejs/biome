---
title: useWhile (since v1.0.0)
---

**Diagnostic Category: `lint/style/useWhile`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Enforce the use of `while` loops instead of `for` loops when the initializer and update expressions are not needed.

## Examples

### Invalid

```jsx
for (; x.running;) {
    x.step();
}
```

<pre class="language-text"><code class="language-text">style/useWhile.js:1:1 <a href="https://biomejs.dev/linter/rules/use-while">lint/style/useWhile</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>while</strong></span><span style="color: Tomato;"> loop instead of a </span><span style="color: Tomato;"><strong>for</strong></span><span style="color: Tomato;"> loop.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>for (; x.running;) {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    x.step();
    <strong>3 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Prefer a </span><span style="color: lightgreen;"><strong>while</strong></span><span style="color: lightgreen;"> loop over a </span><span style="color: lightgreen;"><strong>for</strong></span><span style="color: lightgreen;"> loop without initialization and update.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use a </span><span style="color: lightgreen;"><strong>while</strong></span><span style="color: lightgreen;"> loop.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>f</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;"><strong>;</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;">x</span><span style="color: Tomato;">.</span><span style="color: Tomato;">r</span><span style="color: Tomato;">u</span><span style="color: Tomato;">n</span><span style="color: Tomato;">n</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">g</span><span style="color: Tomato;"><strong>;</strong></span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>w</strong></span><span style="color: MediumSeaGreen;"><strong>h</strong></span><span style="color: MediumSeaGreen;"><strong>i</strong></span><span style="color: MediumSeaGreen;"><strong>l</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">.</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">g</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>      x.step();
    <strong>3</strong> <strong>3</strong><strong> │ </strong>  }
  
</code></pre>

### Valid

```jsx
for(let x = 0; x < 10; i++) {}
```

```jsx
let x = 0
for(; x < 10; i++) {}
```

```jsx
for(let x = 0; x < 10;) {
    i++
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
