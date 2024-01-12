---
title: noRedeclare (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noRedeclare`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://typescript-eslint.io/rules/no-redeclare" target="_blank"><code>no-redeclare</code></a>

Disallow variable, function, class, and type redeclarations in the same scope.

## Examples

### Invalid

```jsx
var a = 3;
var a = 10;
```

<pre class="language-text"><code class="language-text">suspicious/noRedeclare.js:2:5 <a href="https://biomejs.dev/linter/rules/no-redeclare">lint/suspicious/noRedeclare</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Shouldn't redeclare 'a'. Consider to delete it or rename it.</span>
  
    <strong>1 │ </strong>var a = 3;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>var a = 10;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'a' is defined here:</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var a = 3;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>var a = 10;
    <strong>3 │ </strong>
  
</code></pre>

```jsx
let a = 3;
let a = 10;
```

<pre class="language-text"><code class="language-text">suspicious/noRedeclare.js:2:5 <a href="https://biomejs.dev/linter/rules/no-redeclare">lint/suspicious/noRedeclare</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Shouldn't redeclare 'a'. Consider to delete it or rename it.</span>
  
    <strong>1 │ </strong>let a = 3;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>let a = 10;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'a' is defined here:</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let a = 3;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>let a = 10;
    <strong>3 │ </strong>
  
</code></pre>

```jsx
function f() {}
function f() {}
```

<pre class="language-text"><code class="language-text">suspicious/noRedeclare.js:2:10 <a href="https://biomejs.dev/linter/rules/no-redeclare">lint/suspicious/noRedeclare</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Shouldn't redeclare 'f'. Consider to delete it or rename it.</span>
  
    <strong>1 │ </strong>function f() {}
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>function f() {}
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'f' is defined here:</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f() {}
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>function f() {}
    <strong>3 │ </strong>
  
</code></pre>

```jsx
class C {
    static {
        var c = 3;
        var c = 10;
    }
}
```

<pre class="language-text"><code class="language-text">suspicious/noRedeclare.js:4:13 <a href="https://biomejs.dev/linter/rules/no-redeclare">lint/suspicious/noRedeclare</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Shouldn't redeclare 'c'. Consider to delete it or rename it.</span>
  
    <strong>2 │ </strong>    static {
    <strong>3 │ </strong>        var c = 3;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>        var c = 10;
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>    }
    <strong>6 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'c' is defined here:</span>
  
    <strong>1 │ </strong>class C {
    <strong>2 │ </strong>    static {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        var c = 3;
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>        var c = 10;
    <strong>5 │ </strong>    }
  
</code></pre>

```ts
type Person = { name: string; }
class Person { name: string; }
```

<pre class="language-text"><code class="language-text">suspicious/noRedeclare.js:2:7 <a href="https://biomejs.dev/linter/rules/no-redeclare">lint/suspicious/noRedeclare</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Shouldn't redeclare 'Person'. Consider to delete it or rename it.</span>
  
    <strong>1 │ </strong>type Person = { name: string; }
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>class Person { name: string; }
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">'Person' is defined here:</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>type Person = { name: string; }
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>class Person { name: string; }
    <strong>3 │ </strong>
  
</code></pre>

### Valid

```jsx
var a = 3;
a = 10;
```

```ts
class Foo {
    bar(a: A);
    bar(a: A, b: B);
    bar(a: A, b: B) {}
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
