---
title: noConstEnum (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noConstEnum`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Disallow TypeScript `const enum`

Const enums are enums that should be inlined at use sites.
Const enums are not supported by bundlers and are incompatible with the `isolatedModules` mode.
Their use can lead to import nonexistent values (because const enums are erased).

Thus, library authors and bundler users should not use const enums.

## Examples

### Invalid

```ts
const enum Status {
  Open,
  Close,
}
```

<pre class="language-text"><code class="language-text">suspicious/noConstEnum.js:1:1 <a href="https://biomejs.dev/linter/rules/no-const-enum">lint/suspicious/noConstEnum</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>enum declaration</strong></span><span style="color: Tomato;"> should not be </span><span style="color: Tomato;"><strong>const</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const enum Status {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  Open,
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  Close,
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>}
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Const enums are not supported by bundlers and are incompatible with the 'isolatedModules' mode. Their use can lead to import inexistent values.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">See </span><span style="color: lightgreen;"><a href="https://www.typescriptlang.org/docs/handbook/enums.html#const-enum-pitfalls">TypeScript Docs</a></span><span style="color: lightgreen;"> for more details.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Turn the </span><span style="color: lightgreen;"><strong>const enum</strong></span><span style="color: lightgreen;"> into a regular </span><span style="color: lightgreen;"><strong>enum</strong></span><span style="color: lightgreen;">.</span>
  
<strong>  </strong><strong>  1 │ </strong><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>enum<span style="opacity: 0.8;">·</span>Status<span style="opacity: 0.8;">·</span>{
<strong>  </strong><strong>    │ </strong><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>             
</code></pre>

### Valid

```ts
enum Status {
  Open,
  Close,
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
