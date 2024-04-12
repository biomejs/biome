---
title: useNodeAssertStrict (since v1.6.0)
---

**Diagnostic Category: `lint/style/useNodeAssertStrict`**

Promotes the usage of `node:assert/strict` over `node:assert`.

If you prefer stricter assertions when using the Node.js assertion module, the package `node:assert/strict` exposes a set of alias for stricter assertions.

## Examples

### Invalid

```jsx
import * as assert from "node:assert"
```

<pre class="language-text"><code class="language-text">style/useNodeAssertStrict.js:1:25 <a href="https://biomejs.dev/linter/rules/use-node-assert-strict">lint/style/useNodeAssertStrict</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Use </span><span style="color: Orange;"><strong>node:assert/strict</strong></span><span style="color: Orange;"> instead.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import * as assert from &quot;node:assert&quot;
   <strong>   │ </strong>                        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The use of stricter assertion is preferred.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Replace with </span><span style="color: lightgreen;"><strong>node:assert/strict</strong></span><span style="color: lightgreen;">.</span>
  
<strong>  </strong><strong>  1 │ </strong>import<span style="opacity: 0.8;">·</span>*<span style="opacity: 0.8;">·</span>as<span style="opacity: 0.8;">·</span>assert<span style="opacity: 0.8;">·</span>from<span style="opacity: 0.8;">·</span>&quot;node:assert<span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">t</span>&quot;
<strong>  </strong><strong>    │ </strong>                                    <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span> 
</code></pre>

### Valid

```jsx
import * as assert from "node:assert/strict"
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
