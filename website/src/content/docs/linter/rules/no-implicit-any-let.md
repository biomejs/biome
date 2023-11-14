---
title: noImplicitAnyLet (since vnext)
---

**Diagnostic Category: `lint/nursery/noImplicitAnyLet`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow use of implicit `any` type on variable declarations.

TypeScript variable declaration without any type annotation and initialization have the `any` type.
The any type in TypeScript is a dangerous “escape hatch” from the type system.
Using any disables many type checking rules and is generally best used only as a last resort or when prototyping code.
TypeScript’s `--noImplicitAny` compiler option doesn't report this case.

Source: https://www.typescriptlang.org/tsconfig#noImplicitAny

## Examples

### Invalid

```ts
var a;
a = 2;
```

<pre class="language-text"><code class="language-text">nursery/noImplicitAnyLet.js:1:5 <a href="https://biomejs.dev/lint/rules/no-implicit-any-let">lint/nursery/noImplicitAnyLet</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This variable has implicitly the </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> type.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var a;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>a = 2;
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Variable declarations without type annotation and initialization have implicitly the </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;"> type. Declare type or initialize the variable with some value.</span>
  
</code></pre>

```ts
let b;
b = 1
```

<pre class="language-text"><code class="language-text">nursery/noImplicitAnyLet.js:1:5 <a href="https://biomejs.dev/lint/rules/no-implicit-any-let">lint/nursery/noImplicitAnyLet</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This variable has implicitly the </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> type.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let b;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>b = 1
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Variable declarations without type annotation and initialization have implicitly the </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;"> type. Declare type or initialize the variable with some value.</span>
  
</code></pre>

## Valid

```ts
var a = 1;
let a:number;
var b: number
var b =10;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
