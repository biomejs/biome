---
"@biomejs/biome": patch
---

Fixed [#8885](https://github.com/biomejs/biome/issues/8885): `useExhaustiveDependencies` no longer incorrectly reports variables as unnecessary dependencies when they are derived from expressions containing post/pre-increment operators (`++`/`--`) or compound assignment operators (`+=`, `-=`, etc.).

```js
let renderCount = 0;

export const MyComponent = () => {
    // `count` is now correctly recognized as a required dependency
    // because `renderCount++` can produce different values between renders
    const count = renderCount++;

    useEffect(() => {
        console.log(count);
    }, [count]); // no longer reports `count` as unnecessary
};
```
