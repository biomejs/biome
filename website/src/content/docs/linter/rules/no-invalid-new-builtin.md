---
title: noInvalidNewBuiltin (since vnext)
---

**Diagnostic Category: `lint/nursery/noInvalidNewBuiltin`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow `new` operators with global non-constructor functions and
non-constructor built-in objects.

Some global functions cannot be called using the new operator and
will throw a `TypeError` if you attempt to do so. These functions are:

- [`Symbol`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/Symbol)
- [`BigInt`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigInt/BigInt)

Several built-in objects cannot be instantiated and will throw a `TypeError`
if you try to execute them as constructors. These objects are:

- [`Math`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math)
- [`JSON`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/JSON)
- [`Reflect`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect)
- [`Atomics`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics)
- [`Intl`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Intl)

Source: https://eslint.org/docs/latest/rules/no-new-native-nonconstructor/

## Examples

### Invalid

```jsx
var symbol = new Symbol('foo');
var bigInt = new BigInt(9007199254740991);
var math = new Math();
var json = new JSON();
var reflect = new Reflect();
var atomics = new Atomics();
var intl = new Intl();
```

<pre class="language-text"><code class="language-text">nursery/noInvalidNewBuiltin.js:1:14 <a href="https://biomejs.dev/lint/rules/no-invalid-new-builtin">lint/nursery/noInvalidNewBuiltin</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>`Symbol`</strong></span><span style="color: Tomato;"> cannot be called as a constructor.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var symbol = new Symbol('foo');
   <strong>   │ </strong>             <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>var bigInt = new BigInt(9007199254740991);
    <strong>3 │ </strong>var math = new Math();
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove </span><span style="color: rgb(38, 148, 255);"><strong>`new`</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
<strong>  </strong><strong>  1 │ </strong>var<span style="opacity: 0.8;">·</span>symbol<span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span><span style="color: Tomato;">n</span><span style="color: Tomato;">e</span><span style="color: Tomato;">w</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>Symbol('foo');
<strong>  </strong><strong>    │ </strong>             <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>              
nursery/noInvalidNewBuiltin.js:2:14 <a href="https://biomejs.dev/lint/rules/no-invalid-new-builtin">lint/nursery/noInvalidNewBuiltin</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>`BigInt`</strong></span><span style="color: Tomato;"> cannot be called as a constructor.</span>
  
    <strong>1 │ </strong>var symbol = new Symbol('foo');
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>var bigInt = new BigInt(9007199254740991);
   <strong>   │ </strong>             <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>var math = new Math();
    <strong>4 │ </strong>var json = new JSON();
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove </span><span style="color: rgb(38, 148, 255);"><strong>`new`</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
<strong>  </strong><strong>  2 │ </strong>var<span style="opacity: 0.8;">·</span>bigInt<span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span><span style="color: Tomato;">n</span><span style="color: Tomato;">e</span><span style="color: Tomato;">w</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>BigInt(9007199254740991);
<strong>  </strong><strong>    │ </strong>             <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>                         
</code></pre>

## Valid

```jsx
var symbol = Symbol('foo');
var bigInt = BigInt(9007199254740991);

// Ignores shadowed
function foo(Symbol) {
    const symbol = new Symbol('foo');
}
function bar(BigInt) {
    const bigInt = new BigInt(9007199254740991);
}
function baz(Math) {
    const math = new Math();
}
function qux(JSON) {
    const json = new JSON();
}
function quux(Reflect) {
    const reflect = new Reflect();
}
function corge(Intl) {
    const intl = new Intl();
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
