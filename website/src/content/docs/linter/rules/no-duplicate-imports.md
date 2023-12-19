---
title: noDuplicateImports (since vnext)
---

**Diagnostic Category: `lint/nursery/noDuplicateImports`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow duplicate module imports

Using a single import statement per module will make the code clearer because you can see everything being imported from that module on one line.

Source: https://eslint.org/docs/latest/rules/no-duplicate-imports

## Examples

### Invalid

```jsx
import { merge } from 'module';
import something from 'another-module';
import { find } from 'module';
```

<pre class="language-text"><code class="language-text">nursery/noDuplicateImports.js:1:11 <a href="https://biomejs.dev/linter/rules/no-duplicate-imports">lint/nursery/noDuplicateImports</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Variable is read here.</span>

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var a = 1;
   <strong>   │ </strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a = 2;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>

<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This note will give you more information.</span>

</code></pre>

## Valid

```jsx
import { merge, find } from 'module';
import something from 'another-module';
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
