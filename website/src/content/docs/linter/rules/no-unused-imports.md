---
title: noUnusedImports (since v1.3.0)
---

**Diagnostic Category: `lint/nursery/noUnusedImports`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow unused imports.

Unused imports might be the result of an incomplete refactoring.
The code fix can remove comments associated with an `import`.
See the last invalid example.

## Examples

### Invalid

```jsx
import A from 'mod';
```

<pre class="language-text"><code class="language-text">nursery/noUnusedImports.js:1:8 <a href="https://biomejs.dev/linter/rules/no-unused-imports">lint/nursery/noUnusedImports</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>import</strong></span><span style="color: Orange;"> is unused.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import A from 'mod';
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unused imports might be the result of an incomplete refactoring.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the unused import.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>A</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>f</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>'</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>d</strong></span><span style="color: Tomato;"><strong>'</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> 
  
</code></pre>

```jsx
import * as A from 'mod';
```

<pre class="language-text"><code class="language-text">nursery/noUnusedImports.js:1:13 <a href="https://biomejs.dev/linter/rules/no-unused-imports">lint/nursery/noUnusedImports</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>import</strong></span><span style="color: Orange;"> is unused.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import * as A from 'mod';
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unused imports might be the result of an incomplete refactoring.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the unused import.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>*</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>A</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>f</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>'</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>d</strong></span><span style="color: Tomato;"><strong>'</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> 
  
</code></pre>

```ts
import { type A, B } from 'mod';

export { B }
```

<pre class="language-text"><code class="language-text">nursery/noUnusedImports.js:1:15 <a href="https://biomejs.dev/linter/rules/no-unused-imports">lint/nursery/noUnusedImports</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>import</strong></span><span style="color: Orange;"> is unused.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import { type A, B } from 'mod';
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
    <strong>3 │ </strong>export { B }
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unused imports might be the result of an incomplete refactoring.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the unused import.</span>
  
<strong>  </strong><strong>  1 │ </strong>import<span style="opacity: 0.8;">·</span>{<span style="opacity: 0.8;">·</span><span style="color: Tomato;">t</span><span style="color: Tomato;">y</span><span style="color: Tomato;">p</span><span style="color: Tomato;">e</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">A</span><span style="color: Tomato;">,</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>B<span style="opacity: 0.8;">·</span>}<span style="opacity: 0.8;">·</span>from<span style="opacity: 0.8;">·</span>'mod';
<strong>  </strong><strong>    │ </strong>         <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>               
</code></pre>

```jsx
// Header comment
import /*inner comment */ A from 'mod'; // Associated comment

// Another header comment
import {
    // A's header comment
    type A, // A's comment
    // B's header comment
    B,
} from 'mod';

export { B }
```

<pre class="language-text"><code class="language-text">nursery/noUnusedImports.js:7:5 parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">'import { type x ident }' are a TypeScript only feature. Convert your file to a TypeScript file or remove the syntax.</span>
  
    <strong>5 │ </strong>import {
    <strong>6 │ </strong>    // A's header comment
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>7 │ </strong>    type A, // A's comment
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>8 │ </strong>    // B's header comment
    <strong>9 │ </strong>    B,
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">TypeScript only syntax</span>
  
</code></pre>

### Valid

```ts
import { A, type B } from 'mod';

function f(arg: B): A {
    return new A(arg);
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
