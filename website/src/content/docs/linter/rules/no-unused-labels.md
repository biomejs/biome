---
title: noUnusedLabels (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/noUnusedLabels`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-unused-labels" target="_blank"><code>no-unused-labels</code></a>

Disallow unused labels.

Labels that are declared and never used are most likely an error due to incomplete refactoring.

## Examples

### Invalid

```jsx
LOOP: for (const x of xs) {
    if (x > 0) {
        break;
    }
    f(x);
}
```

<pre class="language-text"><code class="language-text">correctness/noUnusedLabels.js:1:1 <a href="https://biomejs.dev/linter/rules/no-unused-labels">lint/correctness/noUnusedLabels</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unused </span><span style="color: Tomato;"><strong>label</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>LOOP: for (const x of xs) {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    if (x &gt; 0) {
    <strong>3 │ </strong>        break;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The label is not used by any </span><span style="color: lightgreen;"><strong>break</strong></span><span style="color: lightgreen;"> statement and continue statement.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the unused </span><span style="color: lightgreen;"><strong>label</strong></span><span style="color: lightgreen;">.</span>
  
<strong>  </strong><strong>  1 │ </strong><span style="color: Tomato;">L</span><span style="color: Tomato;">O</span><span style="color: Tomato;">O</span><span style="color: Tomato;">P</span><span style="color: Tomato;">:</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>for<span style="opacity: 0.8;">·</span>(const<span style="opacity: 0.8;">·</span>x<span style="opacity: 0.8;">·</span>of<span style="opacity: 0.8;">·</span>xs)<span style="opacity: 0.8;">·</span>{
<strong>  </strong><strong>    │ </strong><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>                     
</code></pre>

### Valid

```jsx
LOOP: for (const x of xs) {
    if (x > 0) {
        break LOOP;
    }
    f(x);
}
```

```jsx
function nonNegative(n) {
    DEV: assert(n >= 0);
    return n;
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
