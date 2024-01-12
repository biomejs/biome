---
title: noUselessTypeConstraint (since v1.0.0)
---

**Diagnostic Category: `lint/complexity/noUselessTypeConstraint`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://typescript-eslint.io/rules/no-unnecessary-type-constraint" target="_blank"><code>no-unnecessary-type-constraint</code></a>

Disallow using `any` or `unknown` as type constraint.

Generic type parameters (`<T>`) in TypeScript may be **constrained** with [`extends`](https://www.typescriptlang.org/docs/handbook/generics.html#generic-constraints).
A supplied type must then be a subtype of the supplied constraint.
All types are subtypes of `any` and `unknown`.
It is thus useless to extend from `any` or `unknown`.

## Examples

### Invalid

```ts
interface FooAny<T extends any> {}
```

<pre class="language-text"><code class="language-text">complexity/noUselessTypeConstraint.js:1:20 <a href="https://biomejs.dev/linter/rules/no-useless-type-constraint">lint/complexity/noUselessTypeConstraint</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Constraining a type parameter to </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>unknown</strong></span><span style="color: Tomato;"> is useless.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>interface FooAny&lt;T extends any&gt; {}
   <strong>   │ </strong>                   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All types are subtypes of </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;"> and </span><span style="color: lightgreen;"><strong>unknown</strong></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the constraint.</span>
  
<strong>  </strong><strong>  1 │ </strong>interface<span style="opacity: 0.8;">·</span>FooAny&lt;T<span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">s</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;">n</span><span style="color: Tomato;">y</span>&gt;<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>                  <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>    
</code></pre>

```ts
type BarAny<T extends any> = {};
```

<pre class="language-text"><code class="language-text">complexity/noUselessTypeConstraint.js:1:15 <a href="https://biomejs.dev/linter/rules/no-useless-type-constraint">lint/complexity/noUselessTypeConstraint</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Constraining a type parameter to </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>unknown</strong></span><span style="color: Tomato;"> is useless.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>type BarAny&lt;T extends any&gt; = {};
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All types are subtypes of </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;"> and </span><span style="color: lightgreen;"><strong>unknown</strong></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the constraint.</span>
  
<strong>  </strong><strong>  1 │ </strong>type<span style="opacity: 0.8;">·</span>BarAny&lt;T<span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">s</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;">n</span><span style="color: Tomato;">y</span>&gt;<span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span>{};
<strong>  </strong><strong>    │ </strong>             <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>       
</code></pre>

```ts
class BazAny<T extends any> {
}
```

<pre class="language-text"><code class="language-text">complexity/noUselessTypeConstraint.js:1:16 <a href="https://biomejs.dev/linter/rules/no-useless-type-constraint">lint/complexity/noUselessTypeConstraint</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Constraining a type parameter to </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>unknown</strong></span><span style="color: Tomato;"> is useless.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>class BazAny&lt;T extends any&gt; {
   <strong>   │ </strong>               <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>}
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All types are subtypes of </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;"> and </span><span style="color: lightgreen;"><strong>unknown</strong></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the constraint.</span>
  
<strong>  </strong><strong>  1 │ </strong>class<span style="opacity: 0.8;">·</span>BazAny&lt;T<span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">s</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;">n</span><span style="color: Tomato;">y</span>&gt;<span style="opacity: 0.8;">·</span>{
<strong>  </strong><strong>    │ </strong>              <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>   
</code></pre>

```ts
class BazAny {
  quxAny<U extends any>() {}
}
```

<pre class="language-text"><code class="language-text">complexity/noUselessTypeConstraint.js:2:12 <a href="https://biomejs.dev/linter/rules/no-useless-type-constraint">lint/complexity/noUselessTypeConstraint</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Constraining a type parameter to </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>unknown</strong></span><span style="color: Tomato;"> is useless.</span>
  
    <strong>1 │ </strong>class BazAny {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  quxAny&lt;U extends any&gt;() {}
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All types are subtypes of </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;"> and </span><span style="color: lightgreen;"><strong>unknown</strong></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the constraint.</span>
  
<strong>  </strong><strong>  2 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>quxAny&lt;U<span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">s</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;">n</span><span style="color: Tomato;">y</span>&gt;()<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>          <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>      
</code></pre>

```ts
const QuuxAny = <T extends any>() => {};
```

<pre class="language-text"><code class="language-text">complexity/noUselessTypeConstraint.js:1:20 <a href="https://biomejs.dev/linter/rules/no-useless-type-constraint">lint/complexity/noUselessTypeConstraint</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Constraining a type parameter to </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>unknown</strong></span><span style="color: Tomato;"> is useless.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const QuuxAny = &lt;T extends any&gt;() =&gt; {};
   <strong>   │ </strong>                   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All types are subtypes of </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;"> and </span><span style="color: lightgreen;"><strong>unknown</strong></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the constraint.</span>
  
<strong>  </strong><strong>  1 │ </strong>const<span style="opacity: 0.8;">·</span>QuuxAny<span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span>&lt;T<span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">s</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;">n</span><span style="color: Tomato;">y</span>&gt;()<span style="opacity: 0.8;">·</span>=&gt;<span style="opacity: 0.8;">·</span>{};
<strong>  </strong><strong>    │ </strong>                  <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>          
</code></pre>

```ts
function QuuzAny<T extends any>() {}
```

<pre class="language-text"><code class="language-text">complexity/noUselessTypeConstraint.js:1:20 <a href="https://biomejs.dev/linter/rules/no-useless-type-constraint">lint/complexity/noUselessTypeConstraint</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Constraining a type parameter to </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>unknown</strong></span><span style="color: Tomato;"> is useless.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function QuuzAny&lt;T extends any&gt;() {}
   <strong>   │ </strong>                   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All types are subtypes of </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;"> and </span><span style="color: lightgreen;"><strong>unknown</strong></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the constraint.</span>
  
<strong>  </strong><strong>  1 │ </strong>function<span style="opacity: 0.8;">·</span>QuuzAny&lt;T<span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">s</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;">n</span><span style="color: Tomato;">y</span>&gt;()<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>                  <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>      
</code></pre>

```ts
interface FooUnknown<T extends unknown> {}
```

<pre class="language-text"><code class="language-text">complexity/noUselessTypeConstraint.js:1:24 <a href="https://biomejs.dev/linter/rules/no-useless-type-constraint">lint/complexity/noUselessTypeConstraint</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Constraining a type parameter to </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>unknown</strong></span><span style="color: Tomato;"> is useless.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>interface FooUnknown&lt;T extends unknown&gt; {}
   <strong>   │ </strong>                       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All types are subtypes of </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;"> and </span><span style="color: lightgreen;"><strong>unknown</strong></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the constraint.</span>
  
<strong>  </strong><strong>  1 │ </strong>interface<span style="opacity: 0.8;">·</span>FooUnknown&lt;T<span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">s</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">u</span><span style="color: Tomato;">n</span><span style="color: Tomato;">k</span><span style="color: Tomato;">n</span><span style="color: Tomato;">o</span><span style="color: Tomato;">w</span><span style="color: Tomato;">n</span>&gt;<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>                      <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>    
</code></pre>

```ts
type BarUnknown<T extends unknown> = {};
```

<pre class="language-text"><code class="language-text">complexity/noUselessTypeConstraint.js:1:19 <a href="https://biomejs.dev/linter/rules/no-useless-type-constraint">lint/complexity/noUselessTypeConstraint</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Constraining a type parameter to </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>unknown</strong></span><span style="color: Tomato;"> is useless.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>type BarUnknown&lt;T extends unknown&gt; = {};
   <strong>   │ </strong>                  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All types are subtypes of </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;"> and </span><span style="color: lightgreen;"><strong>unknown</strong></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the constraint.</span>
  
<strong>  </strong><strong>  1 │ </strong>type<span style="opacity: 0.8;">·</span>BarUnknown&lt;T<span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">s</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">u</span><span style="color: Tomato;">n</span><span style="color: Tomato;">k</span><span style="color: Tomato;">n</span><span style="color: Tomato;">o</span><span style="color: Tomato;">w</span><span style="color: Tomato;">n</span>&gt;<span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span>{};
<strong>  </strong><strong>    │ </strong>                 <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>       
</code></pre>

```ts
class BazUnknown<T extends unknown> {
}
```ts,expect_diagnostic
class BazUnknown {
  quxUnknown<U extends unknown>() {}
}
```

<pre class="language-text"><code class="language-text">complexity/noUselessTypeConstraint.js:3:4 parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">unterminated template literal</span>
  
    <strong>1 │ </strong>class BazUnknown&lt;T extends unknown&gt; {
    <strong>2 │ </strong>}
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>```ts,expect_diagnostic
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>class BazUnknown {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>  quxUnknown&lt;U extends unknown&gt;() {}
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>}
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>7 │ </strong>
   <strong>   │ </strong>
  
</code></pre>

```ts
const QuuxUnknown = <T extends unknown>() => {};
```

<pre class="language-text"><code class="language-text">complexity/noUselessTypeConstraint.js:1:24 <a href="https://biomejs.dev/linter/rules/no-useless-type-constraint">lint/complexity/noUselessTypeConstraint</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Constraining a type parameter to </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>unknown</strong></span><span style="color: Tomato;"> is useless.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const QuuxUnknown = &lt;T extends unknown&gt;() =&gt; {};
   <strong>   │ </strong>                       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All types are subtypes of </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;"> and </span><span style="color: lightgreen;"><strong>unknown</strong></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the constraint.</span>
  
<strong>  </strong><strong>  1 │ </strong>const<span style="opacity: 0.8;">·</span>QuuxUnknown<span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span>&lt;T<span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">s</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">u</span><span style="color: Tomato;">n</span><span style="color: Tomato;">k</span><span style="color: Tomato;">n</span><span style="color: Tomato;">o</span><span style="color: Tomato;">w</span><span style="color: Tomato;">n</span>&gt;()<span style="opacity: 0.8;">·</span>=&gt;<span style="opacity: 0.8;">·</span>{};
<strong>  </strong><strong>    │ </strong>                      <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>          
</code></pre>

```ts
function QuuzUnknown<T extends unknown>() {}
```

<pre class="language-text"><code class="language-text">complexity/noUselessTypeConstraint.js:1:24 <a href="https://biomejs.dev/linter/rules/no-useless-type-constraint">lint/complexity/noUselessTypeConstraint</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Constraining a type parameter to </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>unknown</strong></span><span style="color: Tomato;"> is useless.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function QuuzUnknown&lt;T extends unknown&gt;() {}
   <strong>   │ </strong>                       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">All types are subtypes of </span><span style="color: lightgreen;"><strong>any</strong></span><span style="color: lightgreen;"> and </span><span style="color: lightgreen;"><strong>unknown</strong></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the constraint.</span>
  
<strong>  </strong><strong>  1 │ </strong>function<span style="opacity: 0.8;">·</span>QuuzUnknown&lt;T<span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">s</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">u</span><span style="color: Tomato;">n</span><span style="color: Tomato;">k</span><span style="color: Tomato;">n</span><span style="color: Tomato;">o</span><span style="color: Tomato;">w</span><span style="color: Tomato;">n</span>&gt;()<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>                      <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>      
</code></pre>

### Valid

```ts
interface Foo<T> {}

type Bar<T> = {};
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
