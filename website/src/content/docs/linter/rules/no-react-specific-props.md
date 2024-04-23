---
title: noReactSpecificProps (since v1.0.0)
---

**Diagnostic Category: `lint/nursery/noReactSpecificProps`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Source: <a href="https://github.com/solidjs-community/eslint-plugin-solid/blob/main/docs/no-react-specific-props.md" target="_blank"><code>no-react-specific-props</code></a>

Prevents React-specific JSX properties from being used.

## Examples

### Invalid

```jsx
<Hello className="John" />
```

<pre class="language-text"><code class="language-text">nursery/noReactSpecificProps.jsx:1:8 <a href="https://biomejs.dev/linter/rules/no-react-specific-props">lint/nursery/noReactSpecificProps</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This JSX property is specific to React.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;Hello className=&quot;John&quot; /&gt;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

### Valid

```jsx
<Hello class="Doe" />
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
