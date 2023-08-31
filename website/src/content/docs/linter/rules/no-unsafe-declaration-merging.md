---
title: noUnsafeDeclarationMerging (since v1.0.0)
---


Disallow unsafe declaration merging between interfaces and classes.

_TypeScript_'s [declaration merging](https://www.typescriptlang.org/docs/handbook/declaration-merging.html) supports merging separate declarations with the same name.

_Declaration merging_ between classes and interfaces is unsafe.
The _TypeScript Compiler_ doesn't check whether properties defined in the interface are initialized in the class.
This can cause lead to _TypeScript_ not detecting code that will cause runtime errors.

Source: https://typescript-eslint.io/rules/no-unsafe-declaration-merging/

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

<pre class="language-text"><code class="language-text">nursery/noUnsafeDeclarationMerging.js:5:7 <a href="https://biomejs.dev/lint/rules/noUnsafeDeclarationMerging">lint/nursery/noUnsafeDeclarationMerging</a> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>class</strong></span><span style="color: Tomato;"> is unsafely merged with an </span><span style="color: Tomato;"><strong>interface</strong></span><span style="color: Tomato;">.</span>
  
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>class Foo {}
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>
    <strong>7 │ </strong>const foo = new Foo();
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The </span><span style="color: rgb(38, 148, 255);"><strong>interface</strong></span><span style="color: rgb(38, 148, 255);"> is declared here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>interface Foo {
   <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    f(): void
    <strong>3 │ </strong>}
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The TypeScript compiler doesn't check whether properties defined in the interface are initialized in the class.</span>
  
</code></pre>

## Valid

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
