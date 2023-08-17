---
title: Lint Rule useNamespaceKeyword
parent: lint/rules/index
---

# useNamespaceKeyword (since v12.0.0)

> This rule is recommended by Rome.

Require using the `namespace` keyword over the `module` keyword to declare TypeScript namespaces.

TypeScript historically allowed a code organization called _namespace_.
[_ECMAScript modules_ are preferred](https://www.typescriptlang.org/docs/handbook/2/modules.html#typescript-namespaces) (`import` / `export`).

For projects still using _namespaces_, it's preferred to use the `namespace` keyword instead of the `module` keyword.
The `module` keyword is deprecated to avoid any confusion with the _ECMAScript modules_ which are often called _modules_.

Note that TypeScript `module` declarations to describe external APIs (`declare module "foo" {}`) are still allowed.

Source: https://typescript-eslint.io/rules/prefer-namespace-keyword

See also: https://www.typescriptlang.org/docs/handbook/namespaces-and-modules.html

## Examples

### Invalid

```ts
module Example {}
```

<pre class="language-text"><code class="language-text">suspicious/useNamespaceKeyword.js:1:1 <a href="https://biomejs.dev/lint/rules/useNamespaceKeyword">lint/suspicious/useNamespaceKeyword</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use the </span><span style="color: Tomato;"><strong>namespace</strong></span><span style="color: Tomato;"> keyword instead of the outdated </span><span style="color: Tomato;"><strong>module</strong></span><span style="color: Tomato;"> keyword.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>module Example {}
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The </span><span style="color: rgb(38, 148, 255);"><strong>module</strong></span><span style="color: rgb(38, 148, 255);"> keyword is deprecated to avoid any confusion with the </span><span style="color: rgb(38, 148, 255);"><strong>ECMAScript modules</strong></span><span style="color: rgb(38, 148, 255);"> which are often called </span><span style="color: rgb(38, 148, 255);"><strong>modules</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use </span><span style="color: rgb(38, 148, 255);"><strong>namespace</strong></span><span style="color: rgb(38, 148, 255);"> instead.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>d</strong></span><span style="color: Tomato;"><strong>u</strong></span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">E</span><span style="color: Tomato;">x</span><span style="color: Tomato;">a</span><span style="color: Tomato;">m</span><span style="color: Tomato;">p</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span><span style="color: Tomato;">}</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>n</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>m</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>p</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>c</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">E</span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;">}</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

## Valid

```ts
namespace Example {}
```

```ts
declare module "foo" {}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
