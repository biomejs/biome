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

The rule will check for the following assertion calls:

- `expect`
- `assert`
- `assertEquals`

If the assertion function is imported, the rule will check if they are imported from:

- `"chai"`
- `"node:assert"`
- `"node:assert/strict"`
- `"bun:test"`
- `"vitest"`
- Deno assertion module URL

Check the [options](#options) if you need to change the defaults.

## Examples

### Invalid

```jsx
describe("describe", () => {
    expect()
})
```

<pre class="language-text"><code class="language-text">nursery/noMisplacedAssertion.js:2:5 <a href="https://biomejs.dev/linter/rules/no-misplaced-assertion">lint/nursery/noMisplacedAssertion</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The assertion isn't inside a </span><span style="color: Orange;"><strong>it()</strong></span><span style="color: Orange;">, </span><span style="color: Orange;"><strong>test()</strong></span><span style="color: Orange;"> or </span><span style="color: Orange;"><strong>Deno.test()</strong></span><span style="color: Orange;"> function call.</span>
  
    <strong>1 │ </strong>describe(&quot;describe&quot;, () =&gt; {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    expect()
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>})
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This will result in unexpected behaviours from your test suite.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Move the assertion inside a </span><span style="color: lightgreen;"><strong>it()</strong></span><span style="color: lightgreen;">, </span><span style="color: lightgreen;"><strong>test()</strong></span><span style="color: lightgreen;"> or </span><span style="color: lightgreen;"><strong>Deno.test()</strong></span><span style="color: lightgreen;"> function call.</span>
  
</code></pre>

```jsx
import assert from "node:assert";
describe("describe", () => {
    assert.equal()
})
```

<pre class="language-text"><code class="language-text">nursery/noMisplacedAssertion.js:3:5 <a href="https://biomejs.dev/linter/rules/no-misplaced-assertion">lint/nursery/noMisplacedAssertion</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The assertion isn't inside a </span><span style="color: Orange;"><strong>it()</strong></span><span style="color: Orange;">, </span><span style="color: Orange;"><strong>test()</strong></span><span style="color: Orange;"> or </span><span style="color: Orange;"><strong>Deno.test()</strong></span><span style="color: Orange;"> function call.</span>
  
    <strong>1 │ </strong>import assert from &quot;node:assert&quot;;
    <strong>2 │ </strong>describe(&quot;describe&quot;, () =&gt; {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>    assert.equal()
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>})
    <strong>5 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This will result in unexpected behaviours from your test suite.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Move the assertion inside a </span><span style="color: lightgreen;"><strong>it()</strong></span><span style="color: lightgreen;">, </span><span style="color: lightgreen;"><strong>test()</strong></span><span style="color: lightgreen;"> or </span><span style="color: lightgreen;"><strong>Deno.test()</strong></span><span style="color: lightgreen;"> function call.</span>
  
</code></pre>

```jsx
import {test, expect} from "bun:test";
expect(1, 2)
```

<pre class="language-text"><code class="language-text">nursery/noMisplacedAssertion.js:2:1 <a href="https://biomejs.dev/linter/rules/no-misplaced-assertion">lint/nursery/noMisplacedAssertion</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The assertion isn't inside a </span><span style="color: Orange;"><strong>it()</strong></span><span style="color: Orange;">, </span><span style="color: Orange;"><strong>test()</strong></span><span style="color: Orange;"> or </span><span style="color: Orange;"><strong>Deno.test()</strong></span><span style="color: Orange;"> function call.</span>
  
    <strong>1 │ </strong>import {test, expect} from &quot;bun:test&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>expect(1, 2)
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This will result in unexpected behaviours from your test suite.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Move the assertion inside a </span><span style="color: lightgreen;"><strong>it()</strong></span><span style="color: lightgreen;">, </span><span style="color: lightgreen;"><strong>test()</strong></span><span style="color: lightgreen;"> or </span><span style="color: lightgreen;"><strong>Deno.test()</strong></span><span style="color: lightgreen;"> function call.</span>
  
</code></pre>

```jsx
import {assertEquals} from "https://deno.land/std@0.220.0/assert/mod.ts";

assertEquals(url.href, "https://deno.land/foo.js");
Deno.test("url test", () => {
    const url = new URL("./foo.js", "https://deno.land/");
});
```

<pre class="language-text"><code class="language-text">nursery/noMisplacedAssertion.js:3:1 <a href="https://biomejs.dev/linter/rules/no-misplaced-assertion">lint/nursery/noMisplacedAssertion</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The assertion isn't inside a </span><span style="color: Orange;"><strong>it()</strong></span><span style="color: Orange;">, </span><span style="color: Orange;"><strong>test()</strong></span><span style="color: Orange;"> or </span><span style="color: Orange;"><strong>Deno.test()</strong></span><span style="color: Orange;"> function call.</span>
  
    <strong>1 │ </strong>import {assertEquals} from &quot;https://deno.land/std@0.220.0/assert/mod.ts&quot;;
    <strong>2 │ </strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>assertEquals(url.href, &quot;https://deno.land/foo.js&quot;);
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>Deno.test(&quot;url test&quot;, () =&gt; {
    <strong>5 │ </strong>    const url = new URL(&quot;./foo.js&quot;, &quot;https://deno.land/&quot;);
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This will result in unexpected behaviours from your test suite.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Move the assertion inside a </span><span style="color: lightgreen;"><strong>it()</strong></span><span style="color: lightgreen;">, </span><span style="color: lightgreen;"><strong>test()</strong></span><span style="color: lightgreen;"> or </span><span style="color: lightgreen;"><strong>Deno.test()</strong></span><span style="color: lightgreen;"> function call.</span>
  
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

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
