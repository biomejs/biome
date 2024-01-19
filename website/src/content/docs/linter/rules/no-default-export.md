---
title: noDefaultExport (since v1.4.0)
---

**Diagnostic Category: `lint/style/noDefaultExport`**

Source: <a href="https://github.com/import-js/eslint-plugin-import/blob/main/docs/rules/no-default-export.md" target="_blank"><code>no-default-export</code></a>

Disallow default exports.

Default exports cannot be easily discovered inside an editor:
They cannot be suggested by the editor when the user tries to import a name.

Also, default exports don't encourage consistency over a code base:
the module that imports the default export must choose a name.
It is likely that different modules use different names.

Moreover, default exports encourage exporting an object that acts as a namespace.
This is a legacy pattern used to mimic CommonJS modules.

For all these reasons, a team may want to disallow default exports.

Note that this rule disallows only default exports in EcmaScript Module.
It ignores CommonJS default exports.

## Examples

### Invalid

```jsx
export default function f() {};
```

<pre class="language-text"><code class="language-text">style/noDefaultExport.js:1:8 <a href="https://biomejs.dev/linter/rules/no-default-export">lint/style/noDefaultExport</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid </span><span style="color: Orange;"><strong>default</strong></span><span style="color: Orange;"> exports.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>export default function f() {};
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Default exports cannot be easily discovered inside an editor and don't encourage the use of consistent names through a code base.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use a named export instead.</span>
  
</code></pre>

```jsx
export default class C {};
```

<pre class="language-text"><code class="language-text">style/noDefaultExport.js:1:8 <a href="https://biomejs.dev/linter/rules/no-default-export">lint/style/noDefaultExport</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid </span><span style="color: Orange;"><strong>default</strong></span><span style="color: Orange;"> exports.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>export default class C {};
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Default exports cannot be easily discovered inside an editor and don't encourage the use of consistent names through a code base.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use a named export instead.</span>
  
</code></pre>

```jsx
export default {
    f() {},
    g() {},
};
```

<pre class="language-text"><code class="language-text">style/noDefaultExport.js:1:8 <a href="https://biomejs.dev/linter/rules/no-default-export">lint/style/noDefaultExport</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid </span><span style="color: Orange;"><strong>default</strong></span><span style="color: Orange;"> exports.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>export default {
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    f() {},
    <strong>3 │ </strong>    g() {},
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Default exports cannot be easily discovered inside an editor and don't encourage the use of consistent names through a code base.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use a named export instead.</span>
  
</code></pre>

```jsx
export { X as default };
```

<pre class="language-text"><code class="language-text">style/noDefaultExport.js:1:15 <a href="https://biomejs.dev/linter/rules/no-default-export">lint/style/noDefaultExport</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid </span><span style="color: Orange;"><strong>default</strong></span><span style="color: Orange;"> exports.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>export { X as default };
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Default exports cannot be easily discovered inside an editor and don't encourage the use of consistent names through a code base.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use a named export instead.</span>
  
</code></pre>

### Valid

```jsx
export function f () {};
export class C {};
export { default as X } from "mod";
```

```js
module.exports = class {};
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
