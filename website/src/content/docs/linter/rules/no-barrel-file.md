---
title: noBarrelFile (not released)
---

**Diagnostic Category: `lint/nursery/noBarrelFile`**

:::danger
This rule hasn't been released yet.
:::

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Source: <a href="https://github.com/thepassle/eslint-plugin-barrel-files/blob/main/docs/rules/avoid-namespace-import.md" target="_blank"><code>avoid-namespace-import</code></a>

Disallow the use of barrel file.

A barrel file is a file that re-exports all of the exports from other files in a directory.
This structure results in the unnecessary loading of many modules, significantly impacting performance in large-scale applications.
Additionally, it complicates the codebase, making it difficult to navigate and understand the project's dependency graph.

## Examples

### Invalid

```jsx
export * from "foo";
export * as bar from "foo";
export { foo } from "foo";
export { foo, type Bar } from "foo";
export { baz, qux } from "foobar";
export { module as module1 } from "./module1";
export { default as module2 } from "./module2";
```

<pre class="language-text"><code class="language-text">nursery/noBarrelFile.js:4:15 parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">export { type ident }'' are a TypeScript only feature. Convert your file to a TypeScript file or remove the syntax.</span>
  
    <strong>2 │ </strong>export * as bar from &quot;foo&quot;;
    <strong>3 │ </strong>export { foo } from &quot;foo&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>export { foo, type Bar } from &quot;foo&quot;;
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>export { baz, qux } from &quot;foobar&quot;;
    <strong>6 │ </strong>export { module as module1 } from &quot;./module1&quot;;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">TypeScript only syntax</span>
  
</code></pre>

### Valid

```ts
export type * from "foo";
export type { foo } from "foo";
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
