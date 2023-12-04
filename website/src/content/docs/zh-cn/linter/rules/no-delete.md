---
title: noDelete (since v1.0.0)
---

**Diagnostic Category: `lint/performance/noDelete`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Disallow the use of the `delete` operator.

The `delete` operator enables the removal of a property from an object.

The `delete` operator should be avoided because it [can prevent some optimizations of _JavaScript_ engines](https://webkit.org/blog/10298/inline-caching-delete/).
Moreover, it can lead to unexpected results.
For instance, deleting an array element [does not change the length of the array](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/delete#deleting_array_elements).

The only legitimate use of `delete` is on an object that behaves like a _map_.
To allow this pattern, this rule does not report `delete` on computed properties that are not literal values.
Consider using [Map](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map) instead of an object.

## Examples

### Invalid

```jsx
const arr = [1, 2, 3];
delete arr[0];
```

<pre class="language-text"><code class="language-text">performance/noDelete.js:2:1 <a href="https://biomejs.dev/linter/rules/no-delete">lint/performance/noDelete</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>delete</strong></span><span style="color: Tomato;"> operator which can impact performance.</span>
  
    <strong>1 │ </strong>const arr = [1, 2, 3];
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>delete arr[0];
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use an </span><span style="color: lightgreen;"><strong>undefined</strong></span><span style="color: lightgreen;"> assignment instead.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  const arr = [1, 2, 3];
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>d</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">r</span><span style="color: Tomato;">[</span><span style="color: Tomato;">0</span><span style="color: Tomato;">]</span><span style="color: Tomato;">;</span>
      <strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">[</span><span style="color: MediumSeaGreen;">0</span><span style="color: MediumSeaGreen;">]</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: MediumSeaGreen;"><strong>=</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: MediumSeaGreen;"><strong>u</strong></span><span style="color: MediumSeaGreen;"><strong>n</strong></span><span style="color: MediumSeaGreen;"><strong>d</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><strong>f</strong></span><span style="color: MediumSeaGreen;"><strong>i</strong></span><span style="color: MediumSeaGreen;"><strong>n</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><strong>d</strong></span><span style="color: MediumSeaGreen;">;</span>
    <strong>3</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>

```jsx
const obj = {a: {b: {c: 123}}};
delete obj.a.b.c;
```

<pre class="language-text"><code class="language-text">performance/noDelete.js:2:1 <a href="https://biomejs.dev/linter/rules/no-delete">lint/performance/noDelete</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>delete</strong></span><span style="color: Tomato;"> operator which can impact performance.</span>
  
    <strong>1 │ </strong>const obj = {a: {b: {c: 123}}};
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>delete obj.a.b.c;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use an </span><span style="color: lightgreen;"><strong>undefined</strong></span><span style="color: lightgreen;"> assignment instead.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  const obj = {a: {b: {c: 123}}};
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>d</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"><strong>j</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;">;</span>
      <strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;"><strong>b</strong></span><span style="color: MediumSeaGreen;"><strong>j</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>b</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>c</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: MediumSeaGreen;"><strong>=</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>u</strong></span><span style="color: MediumSeaGreen;"><strong>n</strong></span><span style="color: MediumSeaGreen;"><strong>d</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><strong>f</strong></span><span style="color: MediumSeaGreen;"><strong>i</strong></span><span style="color: MediumSeaGreen;"><strong>n</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><strong>d</strong></span><span style="color: MediumSeaGreen;">;</span>
    <strong>3</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
const foo = new Set([1,2,3]);
foo.delete(1);
```

```jsx
const map = Object.create(null);
const key = "key"
map[key] = "value"
delete map[key];
```

```jsx
let x = 5;
delete f(); // uncovered by this rule.
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
