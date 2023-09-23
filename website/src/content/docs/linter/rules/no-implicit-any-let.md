---
title: noImplicitAnyLet (since vnext)
---

**Diagnostic Category: `lint/nursery/noImplicitAnyLet`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Restrict use of implicit any type in Typescript.

Typescript variable declaration without any `type` or `initialization` can cause issue later in the code.

Source: https://www.typescriptlang.org/tsconfig#noImplicitAny

## Examples

### Invalid

```ts
var a;
a = 2;
let b;
b = 1
```

<pre class="language-text"><code class="language-text">nursery/noImplicitAnyLet.js:1:5 <a href="https://biomejs.dev/lint/rules/no-implicit-any-let">lint/nursery/noImplicitAnyLet</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Variable </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> has implicitly </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> type</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var a;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>a = 2;
    <strong>3 │ </strong>let b;
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Declare type or initialize the variable with some value</span>
  
nursery/noImplicitAnyLet.js:3:5 <a href="https://biomejs.dev/lint/rules/no-implicit-any-let">lint/nursery/noImplicitAnyLet</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Variable </span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"> has implicitly </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> type</span>
  
    <strong>1 │ </strong>var a;
    <strong>2 │ </strong>a = 2;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>let b;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>b = 1
    <strong>5 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Declare type or initialize the variable with some value</span>
  
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
