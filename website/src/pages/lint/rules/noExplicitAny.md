---
title: Lint Rule noExplicitAny
parent: lint/rules/index
---

# noExplicitAny (since v10.0.0)

> This rule is recommended by Rome.

Disallow the `any` type usage.

The `any` type in TypeScript is a dangerous "escape hatch" from the type system.
Using `any` disables many type checking rules and is generally best used only as a last resort or when prototyping code.

TypeScript's `--noImplicitAny` compiler option prevents an implied `any`,
but doesn't prevent `any` from being explicitly used the way this rule does.

Source: https://typescript-eslint.io/rules/no-explicit-any

## Examples

### Invalid

```ts
let variable: any = 1;
```

<pre class="language-text"><code class="language-text">suspicious/noExplicitAny.js:1:15 <a href="https://biomejs.dev/lint/rules/noExplicitAny">lint/suspicious/noExplicitAny</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;">. Specify a different type.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let variable: any = 1;
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);"><strong>any</strong></span><span style="color: rgb(38, 148, 255);"> disables many type checking rules. Its use should be avoided.</span>
  
</code></pre>

```ts
class SomeClass {
  message: Array<Array<any>>;
}
```

<pre class="language-text"><code class="language-text">suspicious/noExplicitAny.js:2:24 <a href="https://biomejs.dev/lint/rules/noExplicitAny">lint/suspicious/noExplicitAny</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;">. Specify a different type.</span>
  
    <strong>1 │ </strong>class SomeClass {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  message: Array&lt;Array&lt;any&gt;&gt;;
   <strong>   │ </strong>                       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);"><strong>any</strong></span><span style="color: rgb(38, 148, 255);"> disables many type checking rules. Its use should be avoided.</span>
  
</code></pre>

```ts
function fn(param: Array<any>): void {}
```

<pre class="language-text"><code class="language-text">suspicious/noExplicitAny.js:1:26 <a href="https://biomejs.dev/lint/rules/noExplicitAny">lint/suspicious/noExplicitAny</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;">. Specify a different type.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function fn(param: Array&lt;any&gt;): void {}
   <strong>   │ </strong>                         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);"><strong>any</strong></span><span style="color: rgb(38, 148, 255);"> disables many type checking rules. Its use should be avoided.</span>
  
</code></pre>

### Valid

```ts
let variable: number = 1;
let variable2 = 1;
```

```ts
class SomeClass {
  message: Array<Array<unknown>>;
}
```

```ts
function fn(param: Array<Array<unknown>>): Array<unknown> {}
```

```
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
