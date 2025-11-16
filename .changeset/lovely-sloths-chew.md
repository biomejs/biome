---
"@biomejs/biome": patch
---

Improved the detection of the rule `noUnnecessaryConditions`. Now the rule doesn't isn't triggered for variables that are mutated inside a module.

In the following example, `hey` starts as `false`, but then it's assigned to a string. The rule isn't triggered inside the `if` check.

```js
let hey = false;

function test() {
    hey = "string";
}

if (hey) {}

```
