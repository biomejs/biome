---
title: noPrototypeBuiltins (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noPrototypeBuiltins`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-prototype-builtins" target="_blank"><code>no-prototype-builtins</code></a>

Disallow direct use of `Object.prototype` builtins.

ECMAScript 5.1 added `Object.create` which allows the creation of an object with a custom prototype.
This pattern is often used for objects used as Maps. However, this pattern can lead to errors
if something else relies on prototype properties/methods.
Moreover, the methods could be shadowed, this can lead to random bugs and denial of service
vulnerabilities. For example, calling `hasOwnProperty` directly on parsed JSON like `{"hasOwnProperty": 1}` could lead to vulnerabilities.
To avoid subtle bugs like this, you should call these methods from `Object.prototype`.
For example, `foo.isPrototypeof(bar)` should be replaced with `Object.prototype.isPrototypeof.call(foo, "bar")`
As for the `hasOwn` method, `foo.hasOwn("bar")` should be replaced with `Object.hasOwn(foo, "bar")`.

## Examples

### Invalid

```jsx
var invalid = foo.hasOwnProperty("bar");
```

<pre class="language-text"><code class="language-text">suspicious/noPrototypeBuiltins.js:1:19 <a href="https://biomejs.dev/linter/rules/no-prototype-builtins">lint/suspicious/noPrototypeBuiltins</a> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not access Object.prototype method 'hasOwnProperty' from target object.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var invalid = foo.hasOwnProperty(&quot;bar&quot;);
   <strong>   │ </strong>                  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">It's recommended using </span><span style="color: lightgreen;"><strong>Object.hasOwn()</strong></span><span style="color: lightgreen;"> instead of using </span><span style="color: lightgreen;"><strong>Object.hasOwnProperty()</strong></span><span style="color: lightgreen;">.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">See </span><span style="color: lightgreen;"><a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwn">MDN web docs</a></span><span style="color: lightgreen;"> for more details.</span>
  
</code></pre>

```jsx
var invalid = foo.isPrototypeOf(bar);
```

<pre class="language-text"><code class="language-text">suspicious/noPrototypeBuiltins.js:1:19 <a href="https://biomejs.dev/linter/rules/no-prototype-builtins">lint/suspicious/noPrototypeBuiltins</a> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not access Object.prototype method 'isPrototypeOf' from target object.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var invalid = foo.isPrototypeOf(bar);
   <strong>   │ </strong>                  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
var invalid = foo.propertyIsEnumerable("bar");
```

<pre class="language-text"><code class="language-text">suspicious/noPrototypeBuiltins.js:1:19 <a href="https://biomejs.dev/linter/rules/no-prototype-builtins">lint/suspicious/noPrototypeBuiltins</a> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not access Object.prototype method 'propertyIsEnumerable' from target object.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var invalid = foo.propertyIsEnumerable(&quot;bar&quot;);
   <strong>   │ </strong>                  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

### Valid

```jsx
var valid = Object.hasOwn(foo, "bar");
var valid = Object.prototype.isPrototypeOf.call(foo, bar);
var valid = {}.propertyIsEnumerable.call(foo, "bar");
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
