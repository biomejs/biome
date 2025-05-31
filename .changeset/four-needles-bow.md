---
"@biomejs/biome": patch
---

Fixed `useHookAtTopLevel` rule to properly detect React components wrapped in `memo` and `forwardRef`, and correctly handle property accessors in control flow analysis.

The rule now correctly identifies hooks in components like:
```js
const TestMemo = memo(
  forwardRef((props, ref) => {
    useEffect(() => {
      const [test, setTest] = useState(1); // now properly flagged
    }, []);
    return <div ref={ref}>test</div>;
  })
);
```

And properly handles property accessors:
```js
function ReactComponent() {
    const testObj = {
        get print() {
            return "hello" // no longer considered component return
        }
    }
    const callback = useCallback(() => {}, [])
    return <></>
}
```
