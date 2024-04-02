---
title: noDuplicateObjectKeys (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noDuplicateObjectKeys`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-dupe-keys" target="_blank"><code>no-dupe-keys</code></a>

Prevents object literals having more than one property declaration for the same name.

If an object property with the same name is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the object and previous definitions are ignored, which is likely a mistake.

## Examples

### Invalid

```jsx
const obj = {
   	a: 1,
   	a: 2,
}
```

<pre class="language-text"><code class="language-text">suspicious/noDuplicateObjectKeys.js:2:5 <a href="https://biomejs.dev/linter/rules/no-duplicate-object-keys">lint/suspicious/noDuplicateObjectKeys</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This property value named a is later overwritten by an object member with the same name.</span>
  
    <strong>1 │ </strong>const obj = {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>   	a: 1,
   <strong>   │ </strong>   	<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>   	a: 2,
    <strong>4 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Overwritten with this value.</span>
  
    <strong>1 │ </strong>const obj = {
    <strong>2 │ </strong>   	a: 1,
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>   	a: 2,
   <strong>   │ </strong>   	<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>}
    <strong>5 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">If an object property with the same name is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the object and previous definitions are ignored.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove this property value named a</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  const obj = {
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>→ </strong></span></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>:</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>,</strong></span>
    <strong>3</strong> <strong>2</strong><strong> │ </strong>     	a: 2,
    <strong>4</strong> <strong>3</strong><strong> │ </strong>  }
  
</code></pre>

```jsx
const obj = {
   	set a(v) {},
   	a: 2,
}
```

<pre class="language-text"><code class="language-text">suspicious/noDuplicateObjectKeys.js:2:5 <a href="https://biomejs.dev/linter/rules/no-duplicate-object-keys">lint/suspicious/noDuplicateObjectKeys</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This setter named a is later overwritten by an object member with the same name.</span>
  
    <strong>1 │ </strong>const obj = {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>   	set a(v) {},
   <strong>   │ </strong>   	<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>   	a: 2,
    <strong>4 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Overwritten with this value.</span>
  
    <strong>1 │ </strong>const obj = {
    <strong>2 │ </strong>   	set a(v) {},
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>   	a: 2,
   <strong>   │ </strong>   	<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>}
    <strong>5 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">If an object property with the same name is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the object and previous definitions are ignored.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove this setter named a</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  const obj = {
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>→ </strong></span></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>(</strong></span><span style="color: Tomato;"><strong>v</strong></span><span style="color: Tomato;"><strong>)</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>{</strong></span><span style="color: Tomato;"><strong>}</strong></span><span style="color: Tomato;"><strong>,</strong></span>
    <strong>3</strong> <strong>2</strong><strong> │ </strong>     	a: 2,
    <strong>4</strong> <strong>3</strong><strong> │ </strong>  }
  
</code></pre>

### Valid

```jsx
const obj = {
   	a: 1,
   	b: 2,
}
```

```jsx
const obj = {
   	get a() { return 1; },
   	set a(v) {},
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
