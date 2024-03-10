---
title: noForEach (since v1.0.0)
---

**Diagnostic Category: `lint/complexity/noForEach`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/no-array-for-each.md" target="_blank"><code>no-array-for-each</code></a>

Prefer `for...of` statement instead of `Array.forEach`.

Here's a summary of why `forEach` may be disallowed, and why `for...of` is preferred for almost any use-case of `forEach`:

- Performance: Using `forEach` can lead to performance issues, especially when working with large arrays.
When more requirements are added on, `forEach` typically gets chained with other methods like `filter` or `map`, causing multiple iterations over the same Array.
Encouraging for loops discourages chaining and encourages single-iteration logic (e.g. using a continue instead of `filter`).


- Readability: While `forEach` is a simple and concise way to iterate over an array, it can make the code less readable, especially when the callback function is complex.
In contrast, using a for loop or a `for...of` loop can make the code more explicit and easier to read.


- Debugging: `forEach` can make debugging more difficult, because it hides the iteration process.



## Caveat

We consider all objects with a method named `forEach` to be iterable.
This way, this rule applies to all objects with a method called `forEach`, not just `Array` instances.

## Exception for Index Usage

When the index is explicitly used in the `forEach` callback, it is acceptable to use `forEach`. This is because:

- The index is directly available as the second argument in `forEach`, making it convenient for scenarios where the index is necessary.
- In sparse arrays, `forEach` will skip undefined entries, which differs from the behavior of `for...of` with `Object.entries` that includes these entries.
This can be important for certain array operations, particularly in TypeScript environments with strict type checking.

## Examples

### Invalid

```jsx
els.forEach((el) => {
  f(el);
})
```

<pre class="language-text"><code class="language-text">complexity/noForEach.js:1:1 <a href="https://biomejs.dev/linter/rules/no-for-each">lint/complexity/noForEach</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Prefer </span><span style="color: Tomato;"><strong>for...of</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>forEach</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>els.forEach((el) =&gt; {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  f(el);
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>})
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;"><strong>forEach</strong></span><span style="color: lightgreen;"> may lead to performance issues when working with large arrays. When combined with functions like </span><span style="color: lightgreen;"><strong>filter</strong></span><span style="color: lightgreen;"> or </span><span style="color: lightgreen;"><strong>map</strong></span><span style="color: lightgreen;">, this causes multiple iterations over the same type.</span>
  
</code></pre>

```jsx
els["forEach"](el => {
  f(el);
})
```

<pre class="language-text"><code class="language-text">complexity/noForEach.js:1:1 <a href="https://biomejs.dev/linter/rules/no-for-each">lint/complexity/noForEach</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Prefer </span><span style="color: Tomato;"><strong>for...of</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>forEach</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>els[&quot;forEach&quot;](el =&gt; {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  f(el);
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>})
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;"><strong>forEach</strong></span><span style="color: lightgreen;"> may lead to performance issues when working with large arrays. When combined with functions like </span><span style="color: lightgreen;"><strong>filter</strong></span><span style="color: lightgreen;"> or </span><span style="color: lightgreen;"><strong>map</strong></span><span style="color: lightgreen;">, this causes multiple iterations over the same type.</span>
  
</code></pre>

### Valid

```jsx
els.forEach((el, i) => {
  f(el, i)
})
```

```jsx
for (const el of els) {
  f(el);
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
