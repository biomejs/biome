---
title: useGroupedTypeImport (since v1.0.0)
---

**Diagnostic Category: `lint/nursery/useGroupedTypeImport`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Source: <a href="https://typescript-eslint.io/rules/no-import-type-side-effects" target="_blank"><code>no-import-type-side-effects</code></a>

Enforce the use of `import type` when an `import` only has specifiers with `type` qualifier.

The [`--verbatimModuleSyntax`](https://www.typescriptlang.org/tsconfig#verbatimModuleSyntax) _TypeScript_'s compiler option causes _TypeScript_ to do simple and predictable transpilation on `import` declarations.
Namely, it completely removes `import type` and any imported names with the `type` qualifier.

For instance, the following code:

```ts
import { type A, type B } from "mod-1";
import type { C, D } from "mod-2";
```

<pre class="language-text"><code class="language-text">nursery/useGroupedTypeImport.js:1:8 <a href="https://biomejs.dev/linter/rules/use-grouped-type-import">lint/nursery/useGroupedTypeImport</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>type</strong></span><span style="color: Tomato;"> qualifier can be moved just after </span><span style="color: Tomato;"><strong>import</strong></span><span style="color: Tomato;"> to completely remove the </span><span style="color: Tomato;"><strong>import</strong></span><span style="color: Tomato;"> at compile time.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import { type A, type B } from &quot;mod-1&quot;;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>import type { C, D } from &quot;mod-2&quot;;
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Only </span><span style="color: lightgreen;"><strong>import type</strong></span><span style="color: lightgreen;"> are removed at compile time.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>import type</strong></span><span style="color: lightgreen;"> instead.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">i</span><span style="color: Tomato;">m</span><span style="color: Tomato;">p</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>{</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>y</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">A</span><span style="color: Tomato;">,</span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>y</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">B</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">}</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">m</span><span style="color: Tomato;">o</span><span style="color: Tomato;">d</span><span style="color: Tomato;">-</span><span style="color: Tomato;">1</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>t</strong></span><span style="color: MediumSeaGreen;"><strong>y</strong></span><span style="color: MediumSeaGreen;"><strong>p</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>{</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">A</span><span style="color: MediumSeaGreen;">,</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">B</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">}</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">-</span><span style="color: MediumSeaGreen;">1</span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  import type { C, D } from &quot;mod-2&quot;;
    <strong>3</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>

is transpiled to:

```ts
import "mod-1";
```

Note that, an `import` that includes only names qualified with `type` is transpiled to a [side-effect `import`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import#import_a_module_for_its_side_effects_only).
This can be a surprising behavior: most of developers could expect the deletion of the `import`.

This behavior may still be desirable for applying the potential side-effects of the imported module.
In most cases you will not want to leave behind an unnecessary side effect `import`.
In teh remaining cases, it is often preferable to explicitly use a side-effect `import` to apply the side-effects of a module:

```ts
import "mod"; // side-effect import
import type { A, B } from "mod";
```

## Examples

### Invalid

```ts
import { type A } from "mod";
```

<pre class="language-text"><code class="language-text">nursery/useGroupedTypeImport.js:1:8 <a href="https://biomejs.dev/linter/rules/use-grouped-type-import">lint/nursery/useGroupedTypeImport</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>type</strong></span><span style="color: Tomato;"> qualifier can be moved just after </span><span style="color: Tomato;"><strong>import</strong></span><span style="color: Tomato;"> to completely remove the </span><span style="color: Tomato;"><strong>import</strong></span><span style="color: Tomato;"> at compile time.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import { type A } from &quot;mod&quot;;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Only </span><span style="color: lightgreen;"><strong>import type</strong></span><span style="color: lightgreen;"> are removed at compile time.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>import type</strong></span><span style="color: lightgreen;"> instead.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">i</span><span style="color: Tomato;">m</span><span style="color: Tomato;">p</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>y</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">A</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">}</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">m</span><span style="color: Tomato;">o</span><span style="color: Tomato;">d</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>t</strong></span><span style="color: MediumSeaGreen;"><strong>y</strong></span><span style="color: MediumSeaGreen;"><strong>p</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">A</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">}</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```ts
import type { A, B } from "mod";
```

```ts
import { A, type B } from "mod";
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
