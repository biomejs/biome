---
title: Lint Rule noWith
parent: lint/rules/index
---

# noWith (since v12.0.0)

> This rule is recommended by Rome.

Disallow `with` statements in non-strict contexts.

The `with` statement is potentially problematic because it adds members of an object to the current
scope, making it impossible to tell what a variable inside the block actually refers to.

## Examples

### Invalid

```js
function f() {
  with (point) {
    r = Math.sqrt(x * x + y * y); // is r a member of point?
  }
}
```

<pre class="language-text"><code class="language-text">complexity/noWith.js:2:3 <a href="https://biomejs.dev/lint/rules/noWith">lint/complexity/noWith</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected use of </span><span style="color: Tomato;"><strong>with</strong></span><span style="color: Tomato;"> statement.</span>
  
    <strong>1 │ </strong>function f() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  with (point) {
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>    r = Math.sqrt(x * x + y * y); // is r a member of point?
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>  }
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>}
    <strong>6 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The with statement is potentially problematic because it adds members of an object to the current
</span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">scope, making it impossible to tell what a variable inside the block actually refers to.</span>
  
</code></pre>

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
