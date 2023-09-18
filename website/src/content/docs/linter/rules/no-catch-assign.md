---
title: noCatchAssign (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noCatchAssign`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Disallow reassigning exceptions in catch clauses.

Assignment to a `catch` parameter can be misleading and confusing.
It is often unintended and indicative of a programmer error.

Source: https://eslint.org/docs/latest/rules/no-ex-assign

## Examples

### Invalid

```jsx
try {

} catch (e) {
  e;
  e = 10;
}
```

<pre class="language-text"><code class="language-text">suspicious/noCatchAssign.js:5:3 <a href="https://biomejs.dev/linter/rules/no-catch-assign">lint/suspicious/noCatchAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Reassigning a </span><span style="color: Tomato;"><strong>catch parameter</strong></span><span style="color: Tomato;"> is confusing.</span>
  
    <strong>3 │ </strong>} catch (e) {
    <strong>4 │ </strong>  e;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>  e = 10;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>}
    <strong>7 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The </span><span style="color: rgb(38, 148, 255);"><strong>catch parameter</strong></span><span style="color: rgb(38, 148, 255);"> is declared here:</span>
  
    <strong>1 │ </strong>try {
    <strong>2 │ </strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>} catch (e) {
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>  e;
    <strong>5 │ </strong>  e = 10;
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead.</span>
  
</code></pre>

### Valid

```jsx
try {

} catch (e) {
  let e = 10;
  e = 100;
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
