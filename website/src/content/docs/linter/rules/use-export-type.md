---
title: useExportType (since vnext)
---

**Diagnostic Category: `lint/nursery/useExportType`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Promotes the use of `export type` for type-only types.

_TypeScript_ allows specifying a `type` keyword on an `export` to indicate that the `export` doesn't exist at runtime.
This allows transpilers to safely drop exports of types without looking for their definition.

## Examples

### Invalid

```ts
interface I {}
export { I };
```

<pre class="language-text"><code class="language-text">nursery/useExportType.js:2:10 <a href="https://biomejs.dev/linter/rules/use-export-type">lint/nursery/useExportType</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This export is only a type and should thus use </span><span style="color: Tomato;"><strong>export type</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>interface I {}
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>export { I };
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The type is defined here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>interface I {}
   <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>export { I };
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Using </span><span style="color: lightgreen;"><strong>export type</strong></span><span style="color: lightgreen;"> allows transpilers to safely drop exports of types without looking for their definition.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>export type</strong></span><span style="color: lightgreen;">.</span>
  
<strong>  </strong><strong>  2 │ </strong>export<span style="opacity: 0.8;">·</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">y</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">e</span><span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span>{<span style="opacity: 0.8;">·</span>I<span style="opacity: 0.8;">·</span>};
<strong>  </strong><strong>    │ </strong>       <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>      
</code></pre>

```ts
type T = number;
export { T };
```

<pre class="language-text"><code class="language-text">nursery/useExportType.js:2:10 <a href="https://biomejs.dev/linter/rules/use-export-type">lint/nursery/useExportType</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This export is only a type and should thus use </span><span style="color: Tomato;"><strong>export type</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>type T = number;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>export { T };
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The type is defined here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>type T = number;
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>export { T };
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Using </span><span style="color: lightgreen;"><strong>export type</strong></span><span style="color: lightgreen;"> allows transpilers to safely drop exports of types without looking for their definition.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>export type</strong></span><span style="color: lightgreen;">.</span>
  
<strong>  </strong><strong>  2 │ </strong>export<span style="opacity: 0.8;">·</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">y</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">e</span><span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span>{<span style="opacity: 0.8;">·</span>T<span style="opacity: 0.8;">·</span>};
<strong>  </strong><strong>    │ </strong>       <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>      
</code></pre>

```ts
import type { T } from "./mod.js";
export { T };
```

<pre class="language-text"><code class="language-text">nursery/useExportType.js:2:10 <a href="https://biomejs.dev/linter/rules/use-export-type">lint/nursery/useExportType</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This export is only a type and should thus use </span><span style="color: Tomato;"><strong>export type</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>import type { T } from &quot;./mod.js&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>export { T };
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The type is defined here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import type { T } from &quot;./mod.js&quot;;
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>export { T };
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Using </span><span style="color: lightgreen;"><strong>export type</strong></span><span style="color: lightgreen;"> allows transpilers to safely drop exports of types without looking for their definition.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>export type</strong></span><span style="color: lightgreen;">.</span>
  
<strong>  </strong><strong>  2 │ </strong>export<span style="opacity: 0.8;">·</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">y</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">e</span><span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span>{<span style="opacity: 0.8;">·</span>T<span style="opacity: 0.8;">·</span>};
<strong>  </strong><strong>    │ </strong>       <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>      
</code></pre>

## Valid

```jsx
class C {}
function f() {}
export { C, f };
```

This rules applies only to identifiers locally defined.
It doesn't warn against a type exported as a value in re-export clause such as:

```ts
export { TypeA } from "./mod.ts"
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
