---
title: useExhaustiveDependencies (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/useExhaustiveDependencies`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/facebook/react/blob/main/packages/eslint-plugin-react-hooks/README.md" target="_blank"><code>exhaustive-deps</code></a>

Enforce all dependencies are correctly specified in a React hook.

This rule is a port of the rule [react-hooks/exhaustive-deps](https://legacy.reactjs.org/docs/hooks-rules.html#eslint-plugin), and it's meant to target projects that uses React.

If your project _doesn't_ use React (or Preact), **you shouldn't use this rule**.

The rule will inspect the following **known** hooks:

- `useEffect`
- `useLayoutEffect`
- `useInsertionEffect`
- `useCallback`
- `useMemo`
- `useImperativeHandle`
- `useState`
- `useReducer`
- `useRef`
- `useDebugValue`
- `useDeferredValue`
- `useTransition`

If you want to add more hooks to the rule, check the [options](#options).

## Examples

### Invalid

```jsx
import { useEffect } from "react";

function component() {
    let a = 1;
    useEffect(() => {
        console.log(a);
    }, []);
}
```

<pre class="language-text"><code class="language-text">correctness/useExhaustiveDependencies.js:5:5 <a href="https://biomejs.dev/linter/rules/use-exhaustive-dependencies">lint/correctness/useExhaustiveDependencies</a> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This hook does not specify all of its dependencies: a</span>
  
    <strong>3 │ </strong>function component() {
    <strong>4 │ </strong>    let a = 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>    useEffect(() =&gt; {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>        console.log(a);
    <strong>7 │ </strong>    }, []);
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This dependency is not specified in the hook dependency list.</span>
  
    <strong>4 │ </strong>    let a = 1;
    <strong>5 │ </strong>    useEffect(() =&gt; {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>        console.log(a);
   <strong>   │ </strong>                    <strong><span style="color: Tomato;">^</span></strong>
    <strong>7 │ </strong>    }, []);
    <strong>8 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Either include it or remove the dependency array</span>
  
</code></pre>

```jsx
import { useEffect } from "react";

function component() {
    let b = 1;
    useEffect(() => {
    }, [b]);
}
```

<pre class="language-text"><code class="language-text">correctness/useExhaustiveDependencies.js:5:5 <a href="https://biomejs.dev/linter/rules/use-exhaustive-dependencies">lint/correctness/useExhaustiveDependencies</a> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This hook specifies more dependencies than necessary: b</span>
  
    <strong>3 │ </strong>function component() {
    <strong>4 │ </strong>    let b = 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>    useEffect(() =&gt; {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>    }, [b]);
    <strong>7 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This dependency can be removed from the list.</span>
  
    <strong>4 │ </strong>    let b = 1;
    <strong>5 │ </strong>    useEffect(() =&gt; {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>    }, [b]);
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong>
    <strong>7 │ </strong>}
    <strong>8 │ </strong>
  
</code></pre>

```jsx
import { useEffect, useState } from "react";

function component() {
    const [name, setName] = useState();
    useEffect(() => {
        console.log(name);
        setName("");
    }, [name, setName]);
}
```

<pre class="language-text"><code class="language-text">correctness/useExhaustiveDependencies.js:5:5 <a href="https://biomejs.dev/linter/rules/use-exhaustive-dependencies">lint/correctness/useExhaustiveDependencies</a> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This hook specifies more dependencies than necessary: setName</span>
  
    <strong>3 │ </strong>function component() {
    <strong>4 │ </strong>    const [name, setName] = useState();
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>    useEffect(() =&gt; {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>        console.log(name);
    <strong>7 │ </strong>        setName(&quot;&quot;);
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This dependency can be removed from the list.</span>
  
     <strong>6 │ </strong>        console.log(name);
     <strong>7 │ </strong>        setName(&quot;&quot;);
   <strong><span style="color: Tomato;">&gt;</span></strong> <strong>8 │ </strong>    }, [name, setName]);
    <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
     <strong>9 │ </strong>}
    <strong>10 │ </strong>
  
</code></pre>

```jsx
import { useEffect } from "react";

function component() {
    let a = 1;
    const b = a + 1;
    useEffect(() => {
        console.log(b);
    }, []);
}
```

<pre class="language-text"><code class="language-text">correctness/useExhaustiveDependencies.js:6:5 <a href="https://biomejs.dev/linter/rules/use-exhaustive-dependencies">lint/correctness/useExhaustiveDependencies</a> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This hook does not specify all of its dependencies: b</span>
  
    <strong>4 │ </strong>    let a = 1;
    <strong>5 │ </strong>    const b = a + 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>    useEffect(() =&gt; {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>7 │ </strong>        console.log(b);
    <strong>8 │ </strong>    }, []);
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This dependency is not specified in the hook dependency list.</span>
  
    <strong>5 │ </strong>    const b = a + 1;
    <strong>6 │ </strong>    useEffect(() =&gt; {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>7 │ </strong>        console.log(b);
   <strong>   │ </strong>                    <strong><span style="color: Tomato;">^</span></strong>
    <strong>8 │ </strong>    }, []);
    <strong>9 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Either include it or remove the dependency array</span>
  
</code></pre>

### Valid

```jsx
import { useEffect } from "react";

function component() {
    let a = 1;
    useEffect(() => {
        console.log(a);
    }, [a]);
}
```

```jsx
import { useEffect } from "react";

function component() {
    const a = 1;
    useEffect(() => {
        console.log(a);
    });
}
```

```jsx
import { useEffect, useState } from "react";

function component() {
    const [name, setName] = useState();
    useEffect(() => {
        console.log(name);
        setName("");
    }, [name]);
}
```

```jsx
import { useEffect } from "react";
let outer = false;
function component() {
    useEffect(() => {
        outer = true;
    }, []);
}
```

## Options

Allows to specify custom hooks - from libraries or internal projects -
for which dependencies should be checked and/or which are known to have
stable return values.

### Validating dependencies

For every hook for which you want the dependencies to be validated, you
should specify the index of the closure and the index of the
dependencies array to validate against.

#### Example

```json
{
    "//": "...",
    "options": {
        "hooks": [
            { "name": "useLocation", "closureIndex": 0, "dependenciesIndex": 1},
            { "name": "useQuery", "closureIndex": 1, "dependenciesIndex": 0}
        ]
    }
}
```

Given the previous example, your hooks can be used like this:

```jsx
function Foo() {
    const location = useLocation(() => {}, []);
    const query = useQuery([], () => {});
}
```

### Stable results

When a hook is known to have a stable return value (its identity doesn't
change across invocations), that value doesn't need to be specified in
dependency arrays. For example, setters returned by React's `useState`
hook always have the same identity and should be omitted as such.

You can configure custom hooks that return stable results in one of
three ways:

- `"stableResult": true` -- marks the return value as stable. An example
of a React hook that would be configured like this is `useRef()`.
- `"stableResult": [1]` -- expects the return value to be an array and
marks the given index or indices to be stable. An example of a React
hook that would be configured like this is `useState()`.
- `"stableResult": 1` -- shorthand for `"stableResult": [1]`.

#### Example

```json
{
    "//": "...",
    "options": {
        "hooks": [
            { "name": "useDispatch", "stableResult": true }
        ]
    }
}
```

With this configuration, the following is valid:

```jsx
const dispatch = useDispatch();
// No need to list `dispatch` as dependency:
const doAction = useCallback(() => dispatch(someAction()), []);
```

## Preact support

This rule recognizes rules imported from `preact/compat` and
`preact/hooks` and applies the same rules as for React hooks.

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
