---
title: noGlobalObjectCalls (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/noGlobalObjectCalls`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-obj-calls" target="_blank"><code>no-obj-calls</code></a>

Disallow calling global object properties as functions

ECMAScript provides several global objects that are intended to be used as-is.
Some of these objects look as if they could be constructors due their capitalization (such as Math and JSON) but will throw an error if you try to execute them as functions.

The ECMAScript 5 specification makes it clear that both Math and JSON cannot be invoked:
The Math object does not have a [[Call]] internal property; it is not possible to invoke the Math object as a function.

The ECMAScript 2015 specification makes it clear that Reflect cannot be invoked:
The Reflect object also does not have a [[Call]] internal method; it is not possible to invoke the Reflect object as a function.

The ECMAScript 2017 specification makes it clear that Atomics cannot be invoked:
The Atomics object does not have a [[Call]] internal method; it is not possible to invoke the Atomics object as a function.

And the ECMAScript Internationalization API Specification makes it clear that Intl cannot be invoked:
The Intl object does not have a [[Call]] internal method; it is not possible to invoke the Intl object as a function.

## Examples

### Invalid

```jsx
var math = Math();
```

<pre class="language-text"><code class="language-text">correctness/noGlobalObjectCalls.js:1:12 <a href="https://biomejs.dev/linter/rules/no-global-object-calls">lint/correctness/noGlobalObjectCalls</a> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>Math</strong></span><span style="color: Tomato;"> is not a function.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var math = Math();
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
var newMath = new Math();
```

<pre class="language-text"><code class="language-text">correctness/noGlobalObjectCalls.js:1:19 <a href="https://biomejs.dev/linter/rules/no-global-object-calls">lint/correctness/noGlobalObjectCalls</a> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>Math</strong></span><span style="color: Tomato;"> is not a function.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var newMath = new Math();
   <strong>   │ </strong>                  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
var json = JSON();
```

<pre class="language-text"><code class="language-text">correctness/noGlobalObjectCalls.js:1:12 <a href="https://biomejs.dev/linter/rules/no-global-object-calls">lint/correctness/noGlobalObjectCalls</a> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>Json</strong></span><span style="color: Tomato;"> is not a function.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var json = JSON();
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
var newJSON = new JSON();
```

<pre class="language-text"><code class="language-text">correctness/noGlobalObjectCalls.js:1:19 <a href="https://biomejs.dev/linter/rules/no-global-object-calls">lint/correctness/noGlobalObjectCalls</a> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>Json</strong></span><span style="color: Tomato;"> is not a function.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var newJSON = new JSON();
   <strong>   │ </strong>                  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
var reflect = Reflect();
```

<pre class="language-text"><code class="language-text">correctness/noGlobalObjectCalls.js:1:15 <a href="https://biomejs.dev/linter/rules/no-global-object-calls">lint/correctness/noGlobalObjectCalls</a> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>Reflect</strong></span><span style="color: Tomato;"> is not a function.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var reflect = Reflect();
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
var newReflect = new Reflect();
```

<pre class="language-text"><code class="language-text">correctness/noGlobalObjectCalls.js:1:22 <a href="https://biomejs.dev/linter/rules/no-global-object-calls">lint/correctness/noGlobalObjectCalls</a> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>Reflect</strong></span><span style="color: Tomato;"> is not a function.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var newReflect = new Reflect();
   <strong>   │ </strong>                     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
var atomics = Atomics();
```

<pre class="language-text"><code class="language-text">correctness/noGlobalObjectCalls.js:1:15 <a href="https://biomejs.dev/linter/rules/no-global-object-calls">lint/correctness/noGlobalObjectCalls</a> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>Atomics</strong></span><span style="color: Tomato;"> is not a function.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var atomics = Atomics();
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
var newAtomics = new Atomics();
```

<pre class="language-text"><code class="language-text">correctness/noGlobalObjectCalls.js:1:22 <a href="https://biomejs.dev/linter/rules/no-global-object-calls">lint/correctness/noGlobalObjectCalls</a> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>Atomics</strong></span><span style="color: Tomato;"> is not a function.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var newAtomics = new Atomics();
   <strong>   │ </strong>                     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
var intl = Intl();
```

<pre class="language-text"><code class="language-text">correctness/noGlobalObjectCalls.js:1:12 <a href="https://biomejs.dev/linter/rules/no-global-object-calls">lint/correctness/noGlobalObjectCalls</a> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>Intl</strong></span><span style="color: Tomato;"> is not a function.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var intl = Intl();
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
var newIntl = new Intl();
```

<pre class="language-text"><code class="language-text">correctness/noGlobalObjectCalls.js:1:19 <a href="https://biomejs.dev/linter/rules/no-global-object-calls">lint/correctness/noGlobalObjectCalls</a> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>Intl</strong></span><span style="color: Tomato;"> is not a function.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var newIntl = new Intl();
   <strong>   │ </strong>                  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

### Valid

```jsx
function area(r) {
    return Math.PI * r * r;
}

var object = JSON.parse("{}");

var value = Reflect.get({ x: 1, y: 2 }, "x");

var first = Atomics.load(foo, 0);

var segmenterFr = new Intl.Segmenter("fr", { granularity: "word" });
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
