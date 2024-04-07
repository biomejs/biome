---
title: noExtraNonNullAssertion (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noExtraNonNullAssertion`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://typescript-eslint.io/rules/no-extra-non-null-assertion" target="_blank"><code>no-extra-non-null-assertion</code></a>

Prevents the wrong usage of the non-null assertion operator (`!`) in TypeScript files.

>The `!` non-null assertion operator in TypeScript is used to assert that a value's type does not include `null` or `undefined`. Using the operator any more than once on a single value does nothing.


## Examples

### Invalid

```ts
const bar = foo!!.bar;
```

<pre class="language-text"><code class="language-text">suspicious/noExtraNonNullAssertion.js:1:13 <a href="https://biomejs.dev/linter/rules/no-extra-non-null-assertion">lint/suspicious/noExtraNonNullAssertion</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Forbidden extra non-null assertion.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const bar = foo!!.bar;
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove extra non-null assertion.</span>
  
<strong>  </strong><strong>  1 │ </strong>const<span style="opacity: 0.8;">·</span>bar<span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span>foo<span style="color: Tomato;">!</span>!.bar;
<strong>  </strong><strong>    │ </strong>               <span style="color: Tomato;">-</span>      
</code></pre>

```ts
function fn(bar?: { n: number }) {
  return bar!?.n;
}
```

<pre class="language-text"><code class="language-text">suspicious/noExtraNonNullAssertion.js:2:10 <a href="https://biomejs.dev/linter/rules/no-extra-non-null-assertion">lint/suspicious/noExtraNonNullAssertion</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Forbidden extra non-null assertion.</span>
  
    <strong>1 │ </strong>function fn(bar?: { n: number }) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  return bar!?.n;
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove extra non-null assertion.</span>
  
<strong>  </strong><strong>  2 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>return<span style="opacity: 0.8;">·</span>bar<span style="color: Tomato;">!</span>?.n;
<strong>  </strong><strong>    │ </strong>            <span style="color: Tomato;">-</span>    
</code></pre>

```ts
function fn(bar?: { n: number }) {
  return ((bar!))?.();
}
```

<pre class="language-text"><code class="language-text">suspicious/noExtraNonNullAssertion.js:2:12 <a href="https://biomejs.dev/linter/rules/no-extra-non-null-assertion">lint/suspicious/noExtraNonNullAssertion</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Forbidden extra non-null assertion.</span>
  
    <strong>1 │ </strong>function fn(bar?: { n: number }) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  return ((bar!))?.();
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove extra non-null assertion.</span>
  
<strong>  </strong><strong>  2 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>return<span style="opacity: 0.8;">·</span>((bar<span style="color: Tomato;">!</span>))?.();
<strong>  </strong><strong>    │ </strong>              <span style="color: Tomato;">-</span>       
</code></pre>

### Valid

```ts
const bar = foo!.bar;

obj?.string!.trim();

function fn(key: string | null) {
  const obj = {};
  return obj?.[key!];
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
