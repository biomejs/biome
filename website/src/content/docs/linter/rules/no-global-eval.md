---
title: noGlobalEval (since v1.5.0)
---

**Diagnostic Category: `lint/security/noGlobalEval`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-eval" target="_blank"><code>no-eval</code></a>

Disallow the use of global `eval()`.

The `eval()` function evaluates the passed string as a _JavaScript_ code.
The executed code can access and mutate variables in the scope where the function is called.

The use of `eval()` exposes to [security risks and performance issues](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval#never_use_eval!).
If the executed code is somehow affected by a malicious party,
then you may end up executing malicious code with the privileges of the caller.
Moreover, changing variables in the caller's scope is expensive in modern _JavaScript_ interpreters.

## Examples

### Invalid

```jsx
eval("var a = 0");
```

<pre class="language-text"><code class="language-text">security/noGlobalEval.js:1:1 <a href="https://biomejs.dev/linter/rules/no-global-eval">lint/security/noGlobalEval</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>eval()</strong></span><span style="color: Tomato;"> exposes to security risks and performance issues.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>eval(&quot;var a = 0&quot;);
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">See the </span><span style="color: lightgreen;"><a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval#never_use_eval!">MDN web docs</a></span><span style="color: lightgreen;"> for more details.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Refactor the code so that it doesn't need to call </span><span style="color: lightgreen;"><strong>eval()</strong></span><span style="color: lightgreen;">.</span>
  
</code></pre>

```jsx
(0, globalThis.eval)("var a = 0")
```

<pre class="language-text"><code class="language-text">security/noGlobalEval.js:1:5 <a href="https://biomejs.dev/linter/rules/no-global-eval">lint/security/noGlobalEval</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>eval()</strong></span><span style="color: Tomato;"> exposes to security risks and performance issues.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>(0, globalThis.eval)(&quot;var a = 0&quot;)
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">See the </span><span style="color: lightgreen;"><a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval#never_use_eval!">MDN web docs</a></span><span style="color: lightgreen;"> for more details.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Refactor the code so that it doesn't need to call </span><span style="color: lightgreen;"><strong>eval()</strong></span><span style="color: lightgreen;">.</span>
  
</code></pre>

```jsx
f(eval);
```

<pre class="language-text"><code class="language-text">security/noGlobalEval.js:1:3 <a href="https://biomejs.dev/linter/rules/no-global-eval">lint/security/noGlobalEval</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>eval()</strong></span><span style="color: Tomato;"> exposes to security risks and performance issues.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>f(eval);
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">See the </span><span style="color: lightgreen;"><a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval#never_use_eval!">MDN web docs</a></span><span style="color: lightgreen;"> for more details.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Refactor the code so that it doesn't need to call </span><span style="color: lightgreen;"><strong>eval()</strong></span><span style="color: lightgreen;">.</span>
  
</code></pre>

```jsx
const aliasedEval = eval;
```

<pre class="language-text"><code class="language-text">security/noGlobalEval.js:1:21 <a href="https://biomejs.dev/linter/rules/no-global-eval">lint/security/noGlobalEval</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>eval()</strong></span><span style="color: Tomato;"> exposes to security risks and performance issues.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const aliasedEval = eval;
   <strong>   │ </strong>                    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">See the </span><span style="color: lightgreen;"><a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval#never_use_eval!">MDN web docs</a></span><span style="color: lightgreen;"> for more details.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Refactor the code so that it doesn't need to call </span><span style="color: lightgreen;"><strong>eval()</strong></span><span style="color: lightgreen;">.</span>
  
</code></pre>

### Valid

```js
function f(eval) {
    eval("let a = 0;");
}
```

The rule is not able to detect cases where the global object is aliased:

```jsx
let foo = globalThis;
foo.eval("let a = 0;");
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
