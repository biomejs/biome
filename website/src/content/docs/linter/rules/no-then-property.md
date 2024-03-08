---
title: noThenProperty (since v1.5.0)
---

**Diagnostic Category: `lint/suspicious/noThenProperty`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/no-thenable.md" target="_blank"><code>no-thenable</code></a>

Disallow `then` property.

When combining objects with a `then` method (thenable objects) with await expressions or dynamic imports, caution is necessary.
These syntaxes interpret the object's then method as intended for the resolution or rejection of a promise, which can lead to unexpected behavior or errors.

## Examples

### Invalid

```jsx
export {then};
```

<pre class="language-text"><code class="language-text">suspicious/noThenProperty.js:1:9 <a href="https://biomejs.dev/linter/rules/no-then-property">lint/suspicious/noThenProperty</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not export </span><span style="color: Tomato;"><strong>then</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>export {then};
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
const foo = {
    then() {}
};
```

<pre class="language-text"><code class="language-text">suspicious/noThenProperty.js:2:5 <a href="https://biomejs.dev/linter/rules/no-then-property">lint/suspicious/noThenProperty</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not add </span><span style="color: Tomato;"><strong>then</strong></span><span style="color: Tomato;"> to an object.</span>
  
    <strong>1 │ </strong>const foo = {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    then() {}
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>};
    <strong>4 │ </strong>
  
</code></pre>

```jsx
const foo = {
    get then() {}
};
```

<pre class="language-text"><code class="language-text">suspicious/noThenProperty.js:2:9 <a href="https://biomejs.dev/linter/rules/no-then-property">lint/suspicious/noThenProperty</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not add </span><span style="color: Tomato;"><strong>then</strong></span><span style="color: Tomato;"> to an object.</span>
  
    <strong>1 │ </strong>const foo = {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    get then() {}
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>};
    <strong>4 │ </strong>
  
</code></pre>

```jsx
const foo = {
   get then() {}
};
```

<pre class="language-text"><code class="language-text">suspicious/noThenProperty.js:2:8 <a href="https://biomejs.dev/linter/rules/no-then-property">lint/suspicious/noThenProperty</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not add </span><span style="color: Tomato;"><strong>then</strong></span><span style="color: Tomato;"> to an object.</span>
  
    <strong>1 │ </strong>const foo = {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>   get then() {}
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>};
    <strong>4 │ </strong>
  
</code></pre>

```jsx
foo.then = function () {}
```

<pre class="language-text"><code class="language-text">suspicious/noThenProperty.js:1:1 <a href="https://biomejs.dev/linter/rules/no-then-property">lint/suspicious/noThenProperty</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not add </span><span style="color: Tomato;"><strong>then</strong></span><span style="color: Tomato;"> to an object.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>foo.then = function () {}
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
class Foo {
    then() {}
}
```

<pre class="language-text"><code class="language-text">suspicious/noThenProperty.js:2:5 <a href="https://biomejs.dev/linter/rules/no-then-property">lint/suspicious/noThenProperty</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not add </span><span style="color: Tomato;"><strong>then</strong></span><span style="color: Tomato;"> to a class.</span>
  
    <strong>1 │ </strong>class Foo {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    then() {}
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
</code></pre>

```jsx
class Foo {
    static then() {}
}
```

<pre class="language-text"><code class="language-text">suspicious/noThenProperty.js:2:12 <a href="https://biomejs.dev/linter/rules/no-then-property">lint/suspicious/noThenProperty</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not add </span><span style="color: Tomato;"><strong>then</strong></span><span style="color: Tomato;"> to a class.</span>
  
    <strong>1 │ </strong>class Foo {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    static then() {}
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
</code></pre>

### Valid

```jsx
export {then as success};
```

```jsx
const foo = {
    success() {}
};
```

```jsx
class Foo {
    success() {}
}
```

```jsx
const foo = bar.then;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
