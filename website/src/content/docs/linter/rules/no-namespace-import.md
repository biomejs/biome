---
title: noNamespaceImport (since v1.6.0)
---

**Diagnostic Category: `lint/nursery/noNamespaceImport`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Source: <a href="https://github.com/thepassle/eslint-plugin-barrel-files/blob/main/docs/rules/avoid-namespace-import.md" target="_blank"><code>avoid-namespace-import</code></a>

Disallow the use of namespace imports.

Namespace imports might impact the efficiency of tree shaking, a process that removes unused code from bundles.
The effectiveness of tree shaking largely depends on the bundler (e.g., Webpack, Rollup) and its configuration.
Modern bundlers are generally capable of handling namespace imports effectively, but using named imports is recommended for optimal tree shaking and minimizing bundle size.

## Examples

### Invalid

```jsx
import * as foo from "foo";
```

<pre class="language-text"><code class="language-text">nursery/noNamespaceImport.js:1:8 <a href="https://biomejs.dev/linter/rules/no-namespace-import">lint/nursery/noNamespaceImport</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid namespace imports, it can prevent efficient tree shaking and increase bundle size.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import * as foo from &quot;foo&quot;;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use named imports instead.</span>
  
</code></pre>

### Valid

```ts
import { foo } from "foo"
import type { bar } from "bar"
import type * as baz from "baz"
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
