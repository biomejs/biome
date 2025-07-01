---
"@biomejs/biome": patch
---

Type inference is now able to handle logical expressions: `&&`, `||`, and `??`.


## Examples

```ts
// We can now infer that because `true` is truthy, the entire expression
// evaluates to a `Promise`.
true && Promise.reject("logical operator bypass");

// And we know that this doesn't:
false && Promise.reject("logical operator bypass");

// Truthiness, falsiness, and non-nullishness can all be determined on more
// complex expressions as well. So the following also works:
type Nullish = null | undefined;

type Params = {
    booleanOption: boolean | Nullish;
    falsyOption: false | Nullish;
};

function foo({ booleanOption, falsyOption }: Params) {
    // This may be a Promise:
    booleanOption ?? Promise.reject("logical operator bypass");
    
    // But this never is:
    falsyOption && Promise.reject("logical operator bypass");
}
```
