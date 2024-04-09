---
title: noEvolvingAny (since v1.6.3)
---

**Diagnostic Category: `lint/nursery/noEvolvingAny`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow variables from evolving into `any` type through reassignments.

In TypeScript, variables without explicit type annotations can evolve their types based on subsequent assignments.
This behaviour can accidentally lead to variables with an `any` type, weakening type safety.
Just like the `any` type, evolved `any` types disable many type-checking rules and should be avoided to maintain strong type safety.
This rule prevents such cases by ensuring variables do not evolve into `any` type, encouraging explicit type annotations and controlled type evolutions.

## Examples

### Invalid

```ts
let a;
```

<pre class="language-text"><code class="language-text">nursery/noEvolvingAny.js:1:5 <a href="https://biomejs.dev/linter/rules/no-evolving-any">lint/nursery/noEvolvingAny</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This variable's type is not allowed to evolve implicitly, leading to potential </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> types.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let a;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The variable's type may evolve, leading to </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;">. Use explicit type or initialization. Specifying an explicit type or initial value to avoid implicit type evolution.</span>
  
</code></pre>

```ts
const b = [];
```

<pre class="language-text"><code class="language-text">nursery/noEvolvingAny.js:1:7 <a href="https://biomejs.dev/linter/rules/no-evolving-any">lint/nursery/noEvolvingAny</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This variable's type is not allowed to evolve implicitly, leading to potential </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> types.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const b = [];
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The variable's type may evolve, leading to </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;">. Use explicit type or initialization. Specifying an explicit type or initial value to avoid implicit type evolution.</span>
  
</code></pre>

```ts
let c = null;
```

<pre class="language-text"><code class="language-text">nursery/noEvolvingAny.js:1:5 <a href="https://biomejs.dev/linter/rules/no-evolving-any">lint/nursery/noEvolvingAny</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This variable's type is not allowed to evolve implicitly, leading to potential </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> types.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let c = null;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The variable's type may evolve, leading to </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;">. Use explicit type or initialization. Specifying an explicit type or initial value to avoid implicit type evolution.</span>
  
</code></pre>

### Valid

```ts
let a: number;
let b = 1;
var c : string;
var d = "abn";
const e: never[] = [];
const f = [null];
const g = ['1'];
const h = [1];
let workspace: Workspace | null = null;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
