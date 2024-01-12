---
title: noExcessiveCognitiveComplexity (since v1.0.0)
---

**Diagnostic Category: `lint/complexity/noExcessiveCognitiveComplexity`**

Source: <a href="https://github.com/SonarSource/eslint-plugin-sonarjs/blob/HEAD/docs/rules/cognitive-complexity.md" target="_blank"><code>cognitive-complexity</code></a>

Disallow functions that exceed a given Cognitive Complexity score.

The more complexity a function contains, the harder it is to understand
later on.

Reducing complexity helps to make code more maintenable, both by making
it easier to understand as well as by reducing chances of accidental
side-effects when making changes.

This rule calculates a complexity score for every function and disallows
those that exceed a configured complexity threshold (default: 15).

The complexity score is calculated based on the Cognitive Complexity
algorithm: https://redirect.sonarsource.com/doc/cognitive-complexity.html

## Examples

### Invalid

```jsx
function tooComplex() {
    for (let x = 0; x < 10; x++) {
        for (let y = 0; y < 10; y++) {
            for (let z = 0; z < 10; z++) {
                if (x % 2 === 0) {
                    if (y % 2 === 0) {
                        console.log(x > y ? `${x} > ${y}` : `${y} > ${x}`);
                    }
                }
            }
        }
    }
}
```

<pre class="language-text"><code class="language-text">complexity/noExcessiveCognitiveComplexity.js:1:10 <a href="https://biomejs.dev/linter/rules/no-excessive-cognitive-complexity">lint/complexity/noExcessiveCognitiveComplexity</a> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Excessive complexity detected.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function tooComplex() {
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    for (let x = 0; x &lt; 10; x++) {
    <strong>3 │ </strong>        for (let y = 0; y &lt; 10; y++) {
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Please refactor this function to reduce its complexity score from 21 to the max allowed complexity 15.</span>
  
</code></pre>

## Options

Allows to specify the maximum allowed complexity.

```json
{
    "//": "...",
    "options": {
        "maxAllowedComplexity": 15
    }
}
```

The allowed values range from 1 through 254. The default is 15.

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
