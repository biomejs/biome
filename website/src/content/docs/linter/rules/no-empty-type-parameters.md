---
title: noEmptyTypeParameters (since v1.5.0)
---

**Diagnostic Category: `lint/complexity/noEmptyTypeParameters`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Disallow empty type parameters in type aliases and interfaces.

TypeScript permits the use of empty type parameter lists in type alias and interface declarations; however, this practice is generally discouraged.
Allowing empty type parameter lists can lead to unclear or ambiguous code, where the intention of the generic type is not self-evident.
This rule disallows empty type parameter lists in type alias and interface declarations.

## Examples

### Invalid

```ts
interface Foo<> {}
```

<pre class="language-text"><code class="language-text">complexity/noEmptyTypeParameters.js:1:14 <a href="https://biomejs.dev/linter/rules/no-empty-type-parameters">lint/complexity/noEmptyTypeParameters</a> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Using an </span><span style="color: Tomato;"><strong>empty type parameter list</strong></span><span style="color: Tomato;"> is confusing.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>interface Foo&lt;&gt; {}
   <strong>   │ </strong>             <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Remove the empty type parameter list or add a type parameter.</span>
  
</code></pre>

```ts
type Bar<> = {};
```

<pre class="language-text"><code class="language-text">complexity/noEmptyTypeParameters.js:1:9 <a href="https://biomejs.dev/linter/rules/no-empty-type-parameters">lint/complexity/noEmptyTypeParameters</a> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Using an </span><span style="color: Tomato;"><strong>empty type parameter list</strong></span><span style="color: Tomato;"> is confusing.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>type Bar&lt;&gt; = {};
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Remove the empty type parameter list or add a type parameter.</span>
  
</code></pre>

### Valid

```ts
interface Foo {}
```

```ts
type Foo<T> = {
 bar: T;
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
