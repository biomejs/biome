---
title: noEvolvingAny (not released)
---

**Diagnostic Category: `lint/nursery/noEvolvingAny`**

:::danger
This rule hasn't been released yet.
:::

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow variables from evolving into `any` type through reassignments.
In TypeScript, variables without explicit type annotations can evolve their types based on subsequent assignments.
This behavior can inadvertently lead to variables with an `any` type, weakening type safety.
Just like the `any` type, evolved `any` types disable many type checking rules and should be avoided to maintain strong type safety.
This rule prevents such cases by ensuring variables do not evolve into `any` type, encouraging explicit type annotations and controlled type evolutions.

## Examples

### Invalid

```ts
let a;
const b = [];
let c = null;
```

<pre class="language-text"><code class="language-text">nursery/noEvolvingAny.js:1:5 <a href="https://biomejs.dev/linter/rules/no-evolving-any">lint/nursery/noEvolvingAny</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This variable's type is allowed to evolve implicitly, leading to potential </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> types. Specify an explicit type or initialization to avoid implicit type evolution.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let a;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>const b = [];
    <strong>3 │ </strong>let c = null;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Variable's type may evolve, leading to </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;">. Use explicit type or initialization, e.g., 'let x: number;' or 'let x = 0;'.</span>
  
nursery/noEvolvingAny.js:2:7 <a href="https://biomejs.dev/linter/rules/no-evolving-any">lint/nursery/noEvolvingAny</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This variable's type is allowed to evolve implicitly, leading to potential </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> types. Specify an explicit type or initialization to avoid implicit type evolution.</span>
  
    <strong>1 │ </strong>let a;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>const b = [];
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>let c = null;
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Variable's type may evolve, leading to </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;">. Use explicit type or initialization, e.g., 'let x: number;' or 'let x = 0;'.</span>
  
</code></pre>

```ts
let a = 'hello';
const b = ['hello'];
const c = null;
```

<pre class="language-text"><code class="language-text">nursery/noEvolvingAny.js:3:7 <a href="https://biomejs.dev/linter/rules/no-evolving-any">lint/nursery/noEvolvingAny</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This variable's type is allowed to evolve implicitly, leading to potential </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> types. Specify an explicit type or initialization to avoid implicit type evolution.</span>
  
    <strong>1 │ </strong>let a = 'hello';
    <strong>2 │ </strong>const b = ['hello'];
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>const c = null;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Variable's type may evolve, leading to </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;">. Use explicit type or initialization, e.g., 'let x: number;' or 'let x = 0;'.</span>
  
</code></pre>

### Valid

```ts
var a = 1;
let a:number;
var b:number;
var b = 10;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
