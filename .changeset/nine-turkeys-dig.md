---
"@biomejs/biome": patch
---

Added the new nursery lint rule `noJsxLiterals`, which disallows the use of string literals inside JSX.

The rule catches these cases:

```jsx
<>
  <div>test</div> {/* test is invalid */}
  <>test</>
  <div>
    {/* this string is invalid */}
    asdjfl
    test
    foo
  </div>
</>
```
