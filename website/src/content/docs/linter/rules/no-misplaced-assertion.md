---
title: noMisplacedAssertion (not released)
---

**Diagnostic Category: `lint/nursery/noMisplacedAssertion`**

:::danger
This rule hasn't been released yet.
:::

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Inspired from: <a href="https://github.com/jest-community/eslint-plugin-jest/blob/main/docs/rules/no-standalone-expect.md" target="_blank"><code>no-standalone-expect</code></a>

Checks that the assertion function, for example `expect`, is placed inside an `it()` function call.

Placing (and using) the `expect` assertion function can result in unexpected behaviors when executing your testing suite.

By default, the rule will the following assertion functions: `expect` and `assert`.

If `expect` or `assert` are imported, the rule will check if they are imported from `"chai"`, `"node:assert"` and `"node:assert/strict"`. Check the [options](#options) if you need to change the defaults.

## Examples

### Invalid

```jsx
describe("describe", () => {
    expect()
})
```

<pre class="language-text"><code class="language-text">nursery/noMisplacedAssertion.js:2:5 <a href="https://biomejs.dev/linter/rules/no-misplaced-assertion">lint/nursery/noMisplacedAssertion</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The assertion isn't inside a </span><span style="color: Orange;"><strong>it()</strong></span><span style="color: Orange;"> function call.</span>
  
    <strong>1 │ </strong>describe(&quot;describe&quot;, () =&gt; {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    expect()
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>})
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This will result in unexpected behaviours from your test suite.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Move the assertion inside a </span><span style="color: lightgreen;"><strong>it()</strong></span><span style="color: lightgreen;"> function call.</span>
  
</code></pre>

```jsx
import assert from "node:assert";
describe("describe", () => {
    assert.equal()
})
```

<pre class="language-text"><code class="language-text">nursery/noMisplacedAssertion.js:3:5 <a href="https://biomejs.dev/linter/rules/no-misplaced-assertion">lint/nursery/noMisplacedAssertion</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The assertion isn't inside a </span><span style="color: Orange;"><strong>it()</strong></span><span style="color: Orange;"> function call.</span>
  
    <strong>1 │ </strong>import assert from &quot;node:assert&quot;;
    <strong>2 │ </strong>describe(&quot;describe&quot;, () =&gt; {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>    assert.equal()
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>})
    <strong>5 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This will result in unexpected behaviours from your test suite.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Move the assertion inside a </span><span style="color: lightgreen;"><strong>it()</strong></span><span style="color: lightgreen;"> function call.</span>
  
</code></pre>

### Valid

```jsx
import assert from "node:assert";
describe("describe", () => {
    it("it", () => {
        assert.equal()
    })
})
```

```jsx
describe("describe", () => {
    it("it", () => {
        expect()
    })
})
```

## Options

The rule allows to change the name of the assertion function to check, and the modules to inspect in case the function is imported.

```json
{
    "options": {
        "assertionFunctionNames": ["expect"],
        "specifiers": ["somePackage"]
    }
}
```

With the previous configuration, the rule will be triggered if the function `expect` is used _and_ it is imported from the package `"somePackage"`.

```jsx
import {expect} from "somePackage";
describe("describe", () => {
    assert.equal()
})
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
