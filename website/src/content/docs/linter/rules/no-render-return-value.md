---
title: noRenderReturnValue (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/noRenderReturnValue`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Prevent the usage of the return value of `React.render`.

>`ReactDOM.render()` currently returns a reference to the root `ReactComponent` instance. However, using this return value is legacy
and should be avoided because future versions of React may render components asynchronously in some cases.
If you need a reference to the root `ReactComponent` instance, the preferred solution is to attach a [callback ref](https://reactjs.org/docs/refs-and-the-dom.html#callback-refs)
to the root element.


Source: [ReactDOM documentation](https://facebook.github.io/react/docs/react-dom.html#render)

## Examples

### Invalid

```jsx
const foo = ReactDOM.render(<div />, document.body);
```

<pre class="language-text"><code class="language-text">correctness/noRenderReturnValue.js:1:13 <a href="https://biomejs.dev/linter/rules/no-render-return-value">lint/correctness/noRenderReturnValue</a> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not depend on the value returned by the function </span><span style="color: Tomato;"><strong>ReactDOM.render()</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const foo = ReactDOM.render(&lt;div /&gt;, document.body);
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The returned value is legacy and future versions of React might return that value asynchronously.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">Check the </span><span style="color: lightgreen;"><a href="https://facebook.github.io/react/docs/react-dom.html#render">React documentation</a></span><span style="color: lightgreen;"> for more information.</span>
  
</code></pre>

### Valid

```jsx
ReactDOM.render(<div />, document.body);
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
