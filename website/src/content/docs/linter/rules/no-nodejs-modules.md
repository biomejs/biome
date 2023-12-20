---
title: noNodejsModules (since vnext)
---

**Diagnostic Category: `lint/nursery/noNodejsModules`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Forbid the use of Node.js builtin modules. Can be useful for client-side web projects that
do not have access to those modules.

Source: https://github.com/import-js/eslint-plugin-import/blob/main/docs/rules/no-nodejs-modules.md

## Examples

### Invalid

```jsx
import fs from "fs";
import path from "node:path";
```

<pre class="language-text"><code class="language-text">nursery/noNodejsModules.js:1:16 <a href="https://biomejs.dev/linter/rules/no-nodejs-modules">lint/nursery/noNodejsModules</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Using Node.js modules are forbidden.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import fs from &quot;fs&quot;;
   <strong>   │ </strong>               <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>import path from &quot;node:path&quot;;
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Can be useful for client-side web projects that do not have access to those modules.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Remove the import module.</span>
  
</code></pre>

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
