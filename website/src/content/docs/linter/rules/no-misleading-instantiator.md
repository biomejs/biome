---
title: noMisleadingInstantiator (since vnext)
---

**Diagnostic Category: `lint/nursery/noMisleadingInstantiator`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Enforce proper usage of `new` and `constructor`.

In JavaScript, classes utilize the `constructor` method to initialize a new instance. On the other hand, TypeScript interfaces can describe a class type with a `new()` method signature, though this pattern is not commonly seen in real-world code. Developers, especially those new to JavaScript or TypeScript, might occasionally confuse the use of `constructor` with `new`.
This rule triggers warnings in the following scenarios:

- When a class has a method named `new`.
- When an interface defines a method named `constructor` or `new` that returns the interface type.
- When a type alias has a `constructor` method.

You should not use this rule if you intentionally want a class with a `new` method, and you're confident nobody working in your code will mistake it with an `constructor`.

Source: https://typescript-eslint.io/rules/no-misused-new/

## Examples

### Invalid

```ts
interface I {
  new (): I;
  constructor(): void;
}
```

<pre class="language-text"><code class="language-text">nursery/noMisleadingInstantiator.js:2:3 <a href="https://biomejs.dev/linter/rules/no-misleading-instantiator">lint/nursery/noMisleadingInstantiator</a> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't use the </span><span style="color: Tomato;"><strong>new</strong></span><span style="color: Tomato;"> method in interfaces.</span>
  
    <strong>1 │ </strong>interface I {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  new (): I;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>  constructor(): void;
    <strong>4 │ </strong>}
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);"><strong>new</strong></span><span style="color: rgb(38, 148, 255);"> in an interface suggests it's instantiable, which is incorrect. The returned type should different from the constructor's type.</span>
  
</code></pre>

```ts
class C {
  new(): C;
}
```

<pre class="language-text"><code class="language-text">nursery/noMisleadingInstantiator.js:2:3 <a href="https://biomejs.dev/linter/rules/no-misleading-instantiator">lint/nursery/noMisleadingInstantiator</a> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't use the </span><span style="color: Tomato;"><strong>new</strong></span><span style="color: Tomato;"> method in classes.</span>
  
    <strong>1 │ </strong>class C {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  new(): C;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);"><strong>new</strong></span><span style="color: rgb(38, 148, 255);"> is typically used to instantiate objects. In classes, its usage can be misleading.</span>
  
</code></pre>

### Valid

```ts
declare class C {
  constructor();
}

interface I {
  new (): C;
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
