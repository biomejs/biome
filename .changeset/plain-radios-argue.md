---
"@biomejs/biome": patch
---

Fixed [#7704](https://github.com/biomejs/biome/issues/7704): The [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) rule now correctly adds an object dependency when its method is called within the closure.

For example:

```js
function Component(props) {
  useEffect(() => {
    props.foo();
  }, []);
}
```

will now be fixed to:

```js
function Component(props) {
  useEffect(() => {
    props.foo();
  }, [props]);
}
```
