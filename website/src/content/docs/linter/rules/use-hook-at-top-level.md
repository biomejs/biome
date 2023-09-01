---
title: useHookAtTopLevel (since v1.0.0)
---


Enforce that all React hooks are being called from the Top Level
component functions.

To understand why this required see https://reactjs.org/docs/hooks-rules.html#only-call-hooks-at-the-top-level

## Examples

### Invalid

```jsx
function Component1({ a }) {
    if (a == 1) {
        useEffect();
    }
}
```

<pre class="language-text"><code class="language-text">nursery/useHookAtTopLevel.js:3:9 <a href="https://biomejs.dev/linter/rules/use-hook-at-top-level">lint/nursery/useHookAtTopLevel</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This hook is being called conditionally, but all hooks must be called in the exact same order in every component render.</span>
  
    <strong>1 │ </strong>function Component1({ a }) {
    <strong>2 │ </strong>    if (a == 1) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        useEffect();
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>    }
    <strong>5 │ </strong>}
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">For React to preserve state between calls, hooks needs to be called unconditionally and always in the same order.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">See https://reactjs.org/docs/hooks-rules.html#only-call-hooks-at-the-top-level</span>
  
</code></pre>

## Valid

```jsx
function Component1() {
    useEffect();
}
```

## Options

Allows to specify custom hooks - from libraries or internal projects - that can be considered stable.

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

Given the previous example, your hooks be used like this:

```jsx
function Foo() {
    const location = useLocation(() => {}, []);
    const query = useQuery([], () => {});
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
