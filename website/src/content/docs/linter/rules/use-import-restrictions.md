---
title: useImportRestrictions (since v1.0.0)
---

**Diagnostic Category: `lint/nursery/useImportRestrictions`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Inspired from: <a href="https://github.com/uhyo/eslint-plugin-import-access" target="_blank"><code>eslint-plugin-import-access</code></a>

Disallows package private imports.

This rules enforces the following restrictions:

## Package private visibility

All exported symbols, such as types, functions or other things that may be exported, are
considered to be "package private". This means that modules that reside in the same
directory, as well as submodules of those "sibling" modules, are allowed to import them,
while any other modules that are further away in the file system are restricted from
importing them. A symbol's visibility may be extended by re-exporting from an index file.

Notes:

- This rule only applies to relative imports. External dependencies are exempted.
- This rule only applies to imports for JavaScript and TypeScript files. Imports for
resources such as images or CSS files are exempted.

Source: https://github.com/uhyo/eslint-plugin-import-access

## Examples

### Invalid

```jsx
// Attempt to import from `foo.js` from outside its `sub` module.
import { fooPackageVariable } from "./sub/foo.js";
```

<pre class="language-text"><code class="language-text">nursery/useImportRestrictions.js:2:36 <a href="https://biomejs.dev/linter/rules/use-import-restrictions">lint/nursery/useImportRestrictions</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Importing package private symbols is prohibited from outside the module directory.</span>
  
    <strong>1 │ </strong>// Attempt to import from `foo.js` from outside its `sub` module.
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>import { fooPackageVariable } from &quot;./sub/foo.js&quot;;
   <strong>   │ </strong>                                   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Please import from </span><span style="color: lightgreen;"><strong>./sub</strong></span><span style="color: lightgreen;"> instead (you may need to re-export the symbol(s) from </span><span style="color: lightgreen;"><strong>./sub/foo.js</strong></span><span style="color: lightgreen;">).</span>
  
</code></pre>

```jsx
// Attempt to import from `bar.ts` from outside its `aunt` module.
import { barPackageVariable } from "../aunt/bar.ts";
```

<pre class="language-text"><code class="language-text">nursery/useImportRestrictions.js:2:36 <a href="https://biomejs.dev/linter/rules/use-import-restrictions">lint/nursery/useImportRestrictions</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Importing package private symbols is prohibited from outside the module directory.</span>
  
    <strong>1 │ </strong>// Attempt to import from `bar.ts` from outside its `aunt` module.
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>import { barPackageVariable } from &quot;../aunt/bar.ts&quot;;
   <strong>   │ </strong>                                   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Please import from </span><span style="color: lightgreen;"><strong>../aunt</strong></span><span style="color: lightgreen;"> instead (you may need to re-export the symbol(s) from </span><span style="color: lightgreen;"><strong>../aunt/bar.ts</strong></span><span style="color: lightgreen;">).</span>
  
</code></pre>

```jsx
// Assumed to resolve to a JS/TS file.
import { fooPackageVariable } from "./sub/foo";
```

<pre class="language-text"><code class="language-text">nursery/useImportRestrictions.js:2:36 <a href="https://biomejs.dev/linter/rules/use-import-restrictions">lint/nursery/useImportRestrictions</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Importing package private symbols is prohibited from outside the module directory.</span>
  
    <strong>1 │ </strong>// Assumed to resolve to a JS/TS file.
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>import { fooPackageVariable } from &quot;./sub/foo&quot;;
   <strong>   │ </strong>                                   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Please import from </span><span style="color: lightgreen;"><strong>./sub</strong></span><span style="color: lightgreen;"> instead (you may need to re-export the symbol(s) from </span><span style="color: lightgreen;"><strong>./sub/foo</strong></span><span style="color: lightgreen;">).</span>
  
</code></pre>

```jsx
// If the `sub/foo` module is inaccessible, so is its index file.
import { fooPackageVariable } from "./sub/foo/index.js";
```

<pre class="language-text"><code class="language-text">nursery/useImportRestrictions.js:2:36 <a href="https://biomejs.dev/linter/rules/use-import-restrictions">lint/nursery/useImportRestrictions</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Importing package private symbols is prohibited from outside the module directory.</span>
  
    <strong>1 │ </strong>// If the `sub/foo` module is inaccessible, so is its index file.
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>import { fooPackageVariable } from &quot;./sub/foo/index.js&quot;;
   <strong>   │ </strong>                                   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Please import from </span><span style="color: lightgreen;"><strong>./sub/index.js</strong></span><span style="color: lightgreen;"> instead (you may need to re-export the symbol(s) from </span><span style="color: lightgreen;"><strong>./sub/foo/index.js</strong></span><span style="color: lightgreen;">).</span>
  
</code></pre>

### Valid

```jsx
// Imports within the same module are always allowed.
import { fooPackageVariable } from "./foo.js";

// Resources (anything other than JS/TS files) are exempt.
import { barResource } from "../aunt/bar.png";

// A parent index file is accessible like other modules.
import { internal } from "../../index.js";

// If the `sub` module is accessible, so is its index file.
import { subPackageVariable } from "./sub/index.js";

// Library imports are exempt.
import useAsync from "react-use/lib/useAsync";
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
