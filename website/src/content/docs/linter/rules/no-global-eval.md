---
title: noGlobalEval (since vnext)
---

**Diagnostic Category: `lint/nursery/noGlobalEval`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow the use of global `eval()`.

JavaScript’s `eval()` function is potentially dangerous and is often misused. Using `eval()` on untrusted code can open a program up to several different injection attacks. The use of `eval()` in most contexts can be substituted for a better, alternative approach to a problem.

Source: https://eslint.org/docs/latest/rules/no-eval

## Examples

### Invalid

```js
eval("var a = 0");
```

<pre class="language-text"><code class="language-text">nursery/noGlobalEval.js:1:1 <a href="https://biomejs.dev/linter/rules/no-global-eva-">lint/nursery/noGlobalEval</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>eval()</strong> can be harmful.</span>

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>eval("var 1 = 0");
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>

</code></pre>

```js
(0, eval)("var a = 0");

var foo = eval;
foo("var a = 0");

window.eval("var a = 0");
```

<pre class="language-text"><code class="language-text">nursery/noGlobalEval.js:1:5 <a href="https://biomejs.dev/linter/rules/no-global-eva-">lint/nursery/noGlobalEval</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>eval()</strong> can be harmful.</span>

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>(0, eval)("var a = 0");
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>

nursery/noGlobalEval.js:3:11 <a href="https://biomejs.dev/linter/rules/no-global-eva-">lint/nursery/noGlobalEval</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>eval()</strong> can be harmful.</span>

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>var foo = eval;
   <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ foo("var a = 0");</strong>

nursery/noGlobalEval.js:6:1 <a href="https://biomejs.dev/linter/rules/no-global-eva-">lint/nursery/noGlobalEval</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"><strong>eval()</strong> can be harmful.</span>

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>window.eval("var a = 0");
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>7 │ </strong>

</code></pre>

## Valid

```jsx
function(eval) {
  eval("var a = 0");
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
