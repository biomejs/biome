---
title: noUnusedImports (since v1.3.0)
---

**Diagnostic Category: `lint/correctness/noUnusedImports`**

Disallow unused imports.

Unused imports might be the result of an incomplete refactoring.
The code fix can remove comments associated with an `import`.
See the last invalid example.

Note that the leading trivia, e.g., comments or newlines preceding
the unused imports will also be removed. So that comment directives
like `@ts-expect-error` won't be transferred to a wrong place.

## Options

This rule respects the [`jsxRuntime`](https://biomejs.dev/reference/configuration/#javascriptjsxruntime)
setting and will make an exception for React globals if it is set to
`"reactClassic"`.

## Examples

### Invalid

```jsx
import A from 'mod';
```

<pre class="language-text"><code class="language-text">correctness/noUnusedImports.js:1:8 <a href="https://biomejs.dev/linter/rules/no-unused-imports">lint/correctness/noUnusedImports</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>import</strong></span><span style="color: Orange;"> is unused.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import A from 'mod';
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unused imports might be the result of an incomplete refactoring.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the unused import.</span>
  
<strong>  </strong><strong>  1 │ </strong><span style="color: Tomato;">i</span><span style="color: Tomato;">m</span><span style="color: Tomato;">p</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;">t</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">A</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">'</span><span style="color: Tomato;">m</span><span style="color: Tomato;">o</span><span style="color: Tomato;">d</span><span style="color: Tomato;">'</span><span style="color: Tomato;">;</span>
<strong>  </strong><strong>    │ </strong><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>
</code></pre>

```jsx
import * as A from 'mod';
```

<pre class="language-text"><code class="language-text">correctness/noUnusedImports.js:1:13 <a href="https://biomejs.dev/linter/rules/no-unused-imports">lint/correctness/noUnusedImports</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>import</strong></span><span style="color: Orange;"> is unused.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import * as A from 'mod';
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unused imports might be the result of an incomplete refactoring.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the unused import.</span>
  
<strong>  </strong><strong>  1 │ </strong><span style="color: Tomato;">i</span><span style="color: Tomato;">m</span><span style="color: Tomato;">p</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;">t</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">*</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;">s</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">A</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">'</span><span style="color: Tomato;">m</span><span style="color: Tomato;">o</span><span style="color: Tomato;">d</span><span style="color: Tomato;">'</span><span style="color: Tomato;">;</span>
<strong>  </strong><strong>    │ </strong><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>
</code></pre>

```ts
import { type A, B } from 'mod';

export { B }
```

<pre class="language-text"><code class="language-text">correctness/noUnusedImports.js:1:15 <a href="https://biomejs.dev/linter/rules/no-unused-imports">lint/correctness/noUnusedImports</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━

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

<pre class="language-text"><code class="language-text">correctness/noUnusedImports.js:7:5 parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

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
