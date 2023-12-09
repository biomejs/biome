---
title: useNodeImportProtocol (since vnext)
---

**Diagnostic Category: `lint/nursery/useNodeImportProtocol`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Enforces using the `node:` protocol for Node.js builtin modules.

The prefer-node-protocol rule in ESLint enforces the use of the node: protocol
when importing Node.js builtin modules in JavaScript code.
This helps differentiate between built-in modules and third-party ones, improving code clarity.
The rule marks traditional imports like import fs from 'fs'; as invalid,
suggesting the format import fs from 'node:fs'; instead.

Source: https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/prefer-node-protocol.md

## Examples

### Invalid

```jsx
import fs from 'fs';
```

<pre class="language-text"><code class="language-text">nursery/useNodeImportProtocol.js:1:16 <a href="https://biomejs.dev/linter/rules/use-node-import-protocol">lint/nursery/useNodeImportProtocol</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Import from Node.js builtin module &quot;fs&quot; should use the &quot;node:&quot; protocol.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import fs from 'fs';
   <strong>   │ </strong>               <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Please change to &quot;node:fs&quot;.</span>
  
</code></pre>

```jsx
import os from 'os';
```

<pre class="language-text"><code class="language-text">nursery/useNodeImportProtocol.js:1:16 <a href="https://biomejs.dev/linter/rules/use-node-import-protocol">lint/nursery/useNodeImportProtocol</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Import from Node.js builtin module &quot;os&quot; should use the &quot;node:&quot; protocol.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import os from 'os';
   <strong>   │ </strong>               <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Please change to &quot;node:os&quot;.</span>
  
</code></pre>

```jsx
import path from 'path';
```

<pre class="language-text"><code class="language-text">nursery/useNodeImportProtocol.js:1:18 <a href="https://biomejs.dev/linter/rules/use-node-import-protocol">lint/nursery/useNodeImportProtocol</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Import from Node.js builtin module &quot;path&quot; should use the &quot;node:&quot; protocol.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import path from 'path';
   <strong>   │ </strong>                 <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Please change to &quot;node:path&quot;.</span>
  
</code></pre>

### Valid

```jsx
import fs from 'node:fs';

import os from 'node:os';

import path from 'node:path';
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
