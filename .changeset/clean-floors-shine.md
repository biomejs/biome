---
"@biomejs/biome": patch
---
Fix [#4553](https://github.com/biomejs/biome/issues/4553), `noUselessFragments` fix result has invalid syntax for JSX attribute, the follow code will fix:

```jsx
<Suspense fallback={<><span>Loading...</span></>}>
  {children}
</Suspense>;
```

it will fix as:

```jsx
<Suspense fallback={<span>Loading...</span>}>
  {children}
</Suspense>;
```
