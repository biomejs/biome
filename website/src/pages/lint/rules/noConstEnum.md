---
title: Lint Rule noConstEnum
parent: lint/rules/index
---

# noConstEnum (since v11.0.0)

> This rule is recommended by Biome.

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

<pre class="language-text"><code class="language-text">suspicious/noConstEnum.js:1:1 <a href="https://biomejs.dev/lint/rules/noConstEnum">lint/suspicious/noConstEnum</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>enum declaration</strong></span><span style="color: Tomato;"> should not be </span><span style="color: Tomato;"><strong>const</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const enum Status {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  Open,
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  Close,
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>}
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Const enums are not supported by bundlers and are incompatible with the 'isolatedModules' mode. Their use can lead to import inexistent values.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://www.typescriptlang.org/docs/handbook/enums.html#const-enum-pitfalls">TypeScript Docs</a></span><span style="color: rgb(38, 148, 255);"> for more details.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Turn the </span><span style="color: rgb(38, 148, 255);"><strong>const enum</strong></span><span style="color: rgb(38, 148, 255);"> into a regular </span><span style="color: rgb(38, 148, 255);"><strong>enum</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
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
