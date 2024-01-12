---
title: noNewSymbol (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/noNewSymbol`**

Source: <a href="https://eslint.org/docs/latest/rules/no-new-symbol" target="_blank"><code>no-new-symbol</code></a>

Disallow `new` operators with the `Symbol` object.

`Symbol` cannot be instantiated. This results in throwing a `TypeError`.

## Examples

### Invalid

```jsx
var foo = new Symbol('foo');
```

<pre class="language-text"><code class="language-text">correctness/noNewSymbol.js:1:11 <a href="https://biomejs.dev/linter/rules/no-new-symbol">lint/correctness/noNewSymbol</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;"><strong>Symbol</strong></span><span style="color: Orange;"> cannot be called as a constructor.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var foo = new Symbol('foo');
   <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove </span><span style="color: lightgreen;"><strong>new</strong></span><span style="color: lightgreen;">.</span>
  
<strong>  </strong><strong>  1 │ </strong>var<span style="opacity: 0.8;">·</span>foo<span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span><span style="color: Tomato;">n</span><span style="color: Tomato;">e</span><span style="color: Tomato;">w</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>Symbol('foo');
<strong>  </strong><strong>    │ </strong>          <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>              
</code></pre>

### Valid

```jsx
var bar = Symbol('bar');
function baz() {
    function Symbol() { }
    new Symbol();
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
