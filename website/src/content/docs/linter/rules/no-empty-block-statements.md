---
title: noEmptyBlockStatements (since vnext)
---

**Diagnostic Category: `lint/nursery/noEmptyBlockStatements`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow empty block statements and static blocks.

Empty static blocks and block statements, while not technically errors, usually occur due to refactoring that wasn’t completed. They can cause confusion when reading code.

This rule disallows empty block statements and static blocks. This rule ignores block statements which contain a comment (for example, in an empty catch or finally block of a try statement to indicate that execution should continue regardless of errors).
This rule also ignores static blocks which contain a comment.

Source: https://eslint.org/docs/latest/rules/no-empty-static-block/
Source: https://eslint.org/docs/latest/rules/no-empty/

## Examples

### Invalid

```jsx
function foo () {}

const foo = () => {}

function fooWithNestedEmptyBlock() {
    let a = 1;
    function shouldFail(){}
    return a
 }

const fooWithNestedEmptyBlock = () => {
    let a = 1;
    const shouldFail = () => {}
    return a
 }
let someVar;
if (someVar) {
}

while (someVar) {
}

switch(someVar) {
}
try {
    doSomething();
} catch(ex) {

} finally {

}

```

<pre class="language-text"><code class="language-text">nursery/noEmptyBlockStatements.js:1:17 <a href="https://biomejs.dev/lint/rules/no-empty-block-statements">lint/nursery/noEmptyBlockStatements</a> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">No empty blocks allowed.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function foo () {}
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
    <strong>3 │ </strong>const foo = () =&gt; {}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Empty static blocks and block statements, while not technically errors, usually occur due to refactoring that wasn’t completed. They can cause confusion when reading code.</span>
  
nursery/noEmptyBlockStatements.js:3:19 <a href="https://biomejs.dev/lint/rules/no-empty-block-statements">lint/nursery/noEmptyBlockStatements</a> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">No empty blocks allowed.</span>
  
    <strong>1 │ </strong>function foo () {}
    <strong>2 │ </strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>const foo = () =&gt; {}
   <strong>   │ </strong>                  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
    <strong>5 │ </strong>function fooWithNestedEmptyBlock() {
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Empty static blocks and block statements, while not technically errors, usually occur due to refactoring that wasn’t completed. They can cause confusion when reading code.</span>
  
</code></pre>

## Valid

```jsx
function foo () {let a;}

const foo = () => {let a;}

function fooWithComment() {
  // should work
}

const barWithComment = () => {
  // should work
}

function fooWithMultilineComment() {
  /**
   * this should also work
   */
}

const barWithMultilineComment = () => {
  /**
   * this should also work
   */
}


if (foo) {
  // empty
}

while (foo) {
  /* empty */
}

try {
  doSomething();
} catch (ex) {
  // continue regardless of error
}

try {
  doSomething();
} finally {
  /* continue regardless of error */
}

class Foo {
  static {
      bar();
  }
}

class Foo {
  static {
      // comment
  }
}
```

## Options

The rule provides one option that is detailed in the following subsections.

```json
{
    "//": "...",
    "options": {
        "allowEmptyCatch": true
    }
}
```

### allowEmptyCatch

When set to true allows empty catch clauses (that is, which do not contain a comment)

Default: false

Examples of additional correct code for this rule with the { "allowEmptyCatch": true } option:

```jsx
try {
    doSomething();
} catch (ex) {}

try {
    doSomething();
}
catch (ex) {}
finally {
    /* continue regardless of error */
}
```

nursery/noEmptyBlockStatements.js:3:3 <a href="https://biomejs.dev/lint/rules/no-empty-block-statements">lint/nursery/noEmptyBlockStatements</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">No empty blocks allowed.</span>
  
    <strong>1 │ </strong>try {
    <strong>2 │ </strong>    doSomething();
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>} catch (ex) {}
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
    <strong>5 │ </strong>try {
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Empty static blocks and block statements, while not technically errors, usually occur due to refactoring that wasn’t completed. They can cause confusion when reading code.</span>
  
## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
