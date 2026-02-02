---
"@biomejs/biome": patch
---

Fixed [#8907](https://github.com/biomejs/biome/issues/8907): `useExhaustiveDependencies` now correctly recognizes stable hook results (like `useState` setters and `useRef` values) when they are destructured into `let` bindings.

```js
let [a, setA] = useState(0);
let b = useRef("");

useEffect(() => {
  setA(1);
  b.current = "test";
}, []); // ✅ No longer incorrectly reports setA and b as missing dependencies
```
