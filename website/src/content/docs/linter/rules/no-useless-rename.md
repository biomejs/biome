---
title: noUselessRename (since v1.0.0)
---

**Diagnostic Category: `lint/complexity/noUselessRename`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-useless-rename" target="_blank"><code>no-useless-rename</code></a>

Disallow renaming import, export, and destructured assignments to the same name.

ES2015 allows for the renaming of references in import and export statements as well as destructuring assignments.
This gives programmers a concise syntax for performing these operations while renaming these references:

```jsx
import { foo as bar } from "baz";
export { foo as bar };
let { foo: bar } = baz;
```

With this syntax, it is possible to rename a reference to the same name.
This is a completely redundant operation, as this is the same as not renaming at all.

## Examples

### Invalid

```jsx
import { foo as foo } from "bar";
```

<pre class="language-text"><code class="language-text">complexity/noUselessRename.js:1:10 <a href="https://biomejs.dev/linter/rules/no-useless-rename">lint/complexity/noUselessRename</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Useless rename.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import { foo as foo } from &quot;bar&quot;;
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the renaming.</span>
  
<strong>  </strong><strong>  1 │ </strong>import<span style="opacity: 0.8;">·</span>{<span style="opacity: 0.8;">·</span>foo<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">s</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>}<span style="opacity: 0.8;">·</span>from<span style="opacity: 0.8;">·</span>&quot;bar&quot;;
<strong>  </strong><strong>    │ </strong>             <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>             
</code></pre>

```jsx
export { foo as foo };
```

<pre class="language-text"><code class="language-text">complexity/noUselessRename.js:1:10 <a href="https://biomejs.dev/linter/rules/no-useless-rename">lint/complexity/noUselessRename</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Useless rename.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>export { foo as foo };
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the renaming.</span>
  
<strong>  </strong><strong>  1 │ </strong>export<span style="opacity: 0.8;">·</span>{<span style="opacity: 0.8;">·</span>foo<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">s</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>};
<strong>  </strong><strong>    │ </strong>             <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>  
</code></pre>

```jsx
let { foo: foo } = bar;
```

<pre class="language-text"><code class="language-text">complexity/noUselessRename.js:1:7 <a href="https://biomejs.dev/linter/rules/no-useless-rename">lint/complexity/noUselessRename</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Useless rename.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let { foo: foo } = bar;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the renaming.</span>
  
<strong>  </strong><strong>  1 │ </strong>let<span style="opacity: 0.8;">·</span>{<span style="opacity: 0.8;">·</span>foo<span style="color: Tomato;">:</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="opacity: 0.8;">·</span>}<span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span>bar;
<strong>  </strong><strong>    │ </strong>         <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>         
</code></pre>

### Valid

```jsx
import { foo as bar } from "baz";
```

```jsx
export { foo as bar };
```

```jsx
let { foo: bar } = baz;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
