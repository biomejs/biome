---
title: Lint Rule noUnusedLabels
parent: lint/rules/index
---

# noUnusedLabels (since v1.0.0)

> This rule is recommended by Biome.

Disallow unused labels.

Labels that are declared and never used are most likely an error due to incomplete refactoring.

Source: https://eslint.org/docs/latest/rules/no-unused-labels

## Examples

### Invalid

```js
LOOP: for (const x of xs) {
    if (x > 0) {
        break;
    }
    f(x);
}
```

<pre class="language-text"><code class="language-text">correctness/noUnusedLabels.js:1:1 <a href="https://biomejs.dev/lint/rules/noUnusedLabels">lint/correctness/noUnusedLabels</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unused </span><span style="color: Tomato;"><strong>label</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>LOOP: for (const x of xs) {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    if (x &gt; 0) {
    <strong>3 │ </strong>        break;
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the unused </span><span style="color: rgb(38, 148, 255);"><strong>label</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
<strong>  </strong><strong>  1 │ </strong><span style="color: Tomato;">L</span><span style="color: Tomato;">O</span><span style="color: Tomato;">O</span><span style="color: Tomato;">P</span><span style="color: Tomato;">:</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>for<span style="opacity: 0.8;">·</span>(const<span style="opacity: 0.8;">·</span>x<span style="opacity: 0.8;">·</span>of<span style="opacity: 0.8;">·</span>xs)<span style="opacity: 0.8;">·</span>{
<strong>  </strong><strong>    │ </strong><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>                     
</code></pre>

### Valid

```js
LOOP: for (const x of xs) {
    if (x > 0) {
        break LOOP;
    }
    f(x);
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
