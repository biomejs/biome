---
title: noUselessEmptyExport (since v1.0.0)
---

**Diagnostic Category: `lint/complexity/noUselessEmptyExport`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://typescript-eslint.io/rules/no-useless-empty-export" target="_blank"><code>no-useless-empty-export</code></a>

Disallow empty exports that don't change anything in a module file.

An empty `export {}` is sometimes useful to turn a file that would otherwise be a script into a module.
Per the [TypeScript Handbook Modules page](https://www.typescriptlang.org/docs/handbook/modules.html):

>In TypeScript, just as in ECMAScript 2015,
any file containing a top-level import or export is considered a module.
Conversely, a file without any top-level import or export declarations is treated as a script
whose contents are available in the global scope.


However, an `export {}` statement does nothing if there are any other top-level import or export in the file.

## Examples

### Invalid

```jsx
import { A } from "module";
export {};
```

<pre class="language-text"><code class="language-text">complexity/noUselessEmptyExport.js:2:1 <a href="https://biomejs.dev/linter/rules/no-useless-empty-export">lint/complexity/noUselessEmptyExport</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This empty </span><span style="color: Tomato;"><strong>export</strong></span><span style="color: Tomato;"> is useless because there's another </span><span style="color: Tomato;"><strong>export</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>import</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>import { A } from &quot;module&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>export {};
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This </span><span style="color: lightgreen;"><strong>import</strong></span><span style="color: lightgreen;"> makes useless the empty export.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import { A } from &quot;module&quot;;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>export {};
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove this useless empty export.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  import { A } from &quot;module&quot;;
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>{</strong></span><span style="color: Tomato;"><strong>}</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>3</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
export const A = 0;
export {};
```

<pre class="language-text"><code class="language-text">complexity/noUselessEmptyExport.js:2:1 <a href="https://biomejs.dev/linter/rules/no-useless-empty-export">lint/complexity/noUselessEmptyExport</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This empty </span><span style="color: Tomato;"><strong>export</strong></span><span style="color: Tomato;"> is useless because there's another </span><span style="color: Tomato;"><strong>export</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>import</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>export const A = 0;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>export {};
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This </span><span style="color: lightgreen;"><strong>export</strong></span><span style="color: lightgreen;"> makes useless the empty export.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>export const A = 0;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>export {};
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove this useless empty export.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  export const A = 0;
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>{</strong></span><span style="color: Tomato;"><strong>}</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>3</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
export {};
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
