---
title: noInferrableTypes (since v1.0.0)
---

**Diagnostic Category: `lint/style/noInferrableTypes`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://typescript-eslint.io/rules/no-inferrable-types" target="_blank"><code>no-inferrable-types</code></a>

Disallow type annotations for variables, parameters, and class properties initialized with a literal expression.

TypeScript is able to infer the types of parameters, properties, and variables from their default or initial values.
There is no need to use an explicit `:` type annotation for trivially inferred types (boolean, bigint, number, regex, string).
Doing so adds unnecessary verbosity to code making it harder to read.

In contrast to ESLint's rule, this rule allows to use a wide type for `const` declarations.
Moreover, the rule does not recognize `undefined` values, primitive type constructors (String, Number, ...), and `RegExp` type.
These global variables could be shadowed by local ones.

## Examples

### Invalid

```ts
const variable: 1 = 1;
```

<pre class="language-text"><code class="language-text">style/noInferrableTypes.js:1:15 <a href="https://biomejs.dev/linter/rules/no-inferrable-types">lint/style/noInferrableTypes</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This type annotation is trivially inferred from its initialization.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const variable: 1 = 1;
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the type annotation.</span>
  
<strong>  </strong><strong>  1 │ </strong>const<span style="opacity: 0.8;">·</span>variable<span style="color: Tomato;">:</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">1</span><span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span>1;
<strong>  </strong><strong>    │ </strong>              <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>     
</code></pre>

```ts
let variable: number = 1;
```

<pre class="language-text"><code class="language-text">style/noInferrableTypes.js:1:13 <a href="https://biomejs.dev/linter/rules/no-inferrable-types">lint/style/noInferrableTypes</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This type annotation is trivially inferred from its initialization.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let variable: number = 1;
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the type annotation.</span>
  
<strong>  </strong><strong>  1 │ </strong>let<span style="opacity: 0.8;">·</span>variable<span style="color: Tomato;">:</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">n</span><span style="color: Tomato;">u</span><span style="color: Tomato;">m</span><span style="color: Tomato;">b</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span>1;
<strong>  </strong><strong>    │ </strong>            <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>     
</code></pre>

```ts
class SomeClass {
  readonly field: 1 = 1;
}
```

<pre class="language-text"><code class="language-text">style/noInferrableTypes.js:2:17 <a href="https://biomejs.dev/linter/rules/no-inferrable-types">lint/style/noInferrableTypes</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This type annotation is trivially inferred from its initialization.</span>
  
    <strong>1 │ </strong>class SomeClass {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  readonly field: 1 = 1;
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the type annotation.</span>
  
<strong>  </strong><strong>  2 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>readonly<span style="opacity: 0.8;">·</span>field<span style="color: Tomato;">:</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">1</span><span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span>1;
<strong>  </strong><strong>    │ </strong>                <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>     
</code></pre>

```ts
class SomeClass {
  field: number = 1;
}
```

<pre class="language-text"><code class="language-text">style/noInferrableTypes.js:2:8 <a href="https://biomejs.dev/linter/rules/no-inferrable-types">lint/style/noInferrableTypes</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This type annotation is trivially inferred from its initialization.</span>
  
    <strong>1 │ </strong>class SomeClass {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  field: number = 1;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the type annotation.</span>
  
<strong>  </strong><strong>  2 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>field<span style="color: Tomato;">:</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">n</span><span style="color: Tomato;">u</span><span style="color: Tomato;">m</span><span style="color: Tomato;">b</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span>1;
<strong>  </strong><strong>    │ </strong>       <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>     
</code></pre>

```ts
function f(param: number = 1): void {}
```

<pre class="language-text"><code class="language-text">style/noInferrableTypes.js:1:17 <a href="https://biomejs.dev/linter/rules/no-inferrable-types">lint/style/noInferrableTypes</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This type annotation is trivially inferred from its initialization.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f(param: number = 1): void {}
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the type annotation.</span>
  
<strong>  </strong><strong>  1 │ </strong>function<span style="opacity: 0.8;">·</span>f(param<span style="color: Tomato;">:</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">n</span><span style="color: Tomato;">u</span><span style="color: Tomato;">m</span><span style="color: Tomato;">b</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span>1):<span style="opacity: 0.8;">·</span>void<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>                <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>              
</code></pre>

### Valid

```ts
const variable: number = 1;
```

```ts
let variable: 1 | 2 = 1;
```

```ts
class SomeClass {
  readonly field: number = 1;
}
```

```ts
// `undefined` could be shadowed
const variable: undefined = undefined;
```

```ts
// `RegExp` could be shadowed
const variable: RegExp = /a/;
```

```ts
// `String` could be shadowed
let variable: string = String(5);
```

```ts
class SomeClass {
  field: 1 | 2 = 1;
}
```

```ts
function f(param: 1 | 2 = 1): void {}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
