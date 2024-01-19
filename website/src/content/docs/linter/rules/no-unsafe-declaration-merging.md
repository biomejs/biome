---
title: noUnsafeDeclarationMerging (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noUnsafeDeclarationMerging`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://typescript-eslint.io/rules/no-unsafe-declaration-merging" target="_blank"><code>no-unsafe-declaration-merging</code></a>

Disallow unsafe declaration merging between interfaces and classes.

_TypeScript_'s [declaration merging](https://www.typescriptlang.org/docs/handbook/declaration-merging.html) supports merging separate declarations with the same name.

_Declaration merging_ between classes and interfaces is unsafe.
The _TypeScript Compiler_ doesn't check whether properties defined in the interface are initialized in the class.
This can cause lead to _TypeScript_ not detecting code that will cause runtime errors.

## Examples

### Invalid

```ts
interface Foo {
    f(): void
}

class Foo {}

const foo = new Foo();
foo.f(); // Runtime Error: Cannot read properties of undefined.
```

<pre class="language-text"><code class="language-text">suspicious/noUnsafeDeclarationMerging.js:5:7 <a href="https://biomejs.dev/linter/rules/no-unsafe-declaration-merging">lint/suspicious/noUnsafeDeclarationMerging</a> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>class</strong></span><span style="color: Tomato;"> is unsafely merged with an </span><span style="color: Tomato;"><strong>interface</strong></span><span style="color: Tomato;">.</span>
  
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>class Foo {}
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>
    <strong>7 │ </strong>const foo = new Foo();
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The </span><span style="color: lightgreen;"><strong>interface</strong></span><span style="color: lightgreen;"> is declared here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>interface Foo {
   <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    f(): void
    <strong>3 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The TypeScript compiler doesn't check whether properties defined in the interface are initialized in the class.</span>
  
</code></pre>

### Valid

```ts
interface Foo {}
class Bar implements Foo {}
```

```ts
namespace Baz {}
namespace Baz {}
enum Baz {}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
